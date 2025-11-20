use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use async_trait::async_trait;

use crate::core::{
    net::http::HttpClient,
    traits::{ExchangeService, ExchangeStatic},
    types::{
        Asks, Bids, OrderBook, TradingPairs,
        structs::{Network, PriceData, TradingPair},
    },
    utils::{encrypt_hmac_sha256, get_current_timestamp, hex_encode, parse_string_typed_glass},
};
#[derive(Debug)]
pub struct Bybit {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    excluded_trading_pairs: HashSet<String>,
    http_client: HttpClient,
}

impl Bybit {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        excluded_trading_pairs: &[&str],
    ) -> Result<Self, Box<dyn Error>> {
        let mut set: HashSet<String> = HashSet::new();
        for pair in excluded_trading_pairs {
            set.insert(pair.to_string().to_uppercase());
        }
        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
            excluded_trading_pairs: set,
        })
    }
}

#[async_trait]
impl ExchangeService for Bybit {
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn Error>> {
        let networks: Vec<Network> = Vec::new();
        Ok(networks)
    }
    async fn is_margin_available(&self, asset: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    async fn fetch_tickers(&self) -> Option<TradingPairs> {
        let Ok(response) = self
            .http_client
            .get(
                "/v5/market/tickers",
                Some(&[("category".to_string(), "spot".to_string())]),
                None,
            )
            .await
        else {
            return None;
        };
        let mut trading_pairs: TradingPairs = HashMap::new();

        if response["retMsg"] != "OK" {
            return None;
        };
        let tickers = &response["result"]["list"];

        for ticker in tickers.members() {
            let Ok(parsed_ask_price) = ticker["ask1Price"].as_str()?.parse::<f64>() else {
                continue;
            };

            let Ok(parsed_bid_price) = ticker["bid1Price"].as_str()?.parse::<f64>() else {
                continue;
            };

            let Some(trading_pair) = TradingPair::from_str(&ticker["symbol"].to_string()) else {
                continue;
            };

            trading_pairs.insert(
                trading_pair,
                PriceData::new(
                    (self.name.clone(), parsed_bid_price),
                    (self.name.clone(), parsed_ask_price),
                ),
            );
        }
        Some(trading_pairs)
    }
    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let recv_window = 5000;
        let order_book_limit = 500;
        let query_string = format!(
            "category={}&symbol={}&limit={}",
            "spot",
            base.to_string() + quote,
            order_book_limit,
        );
        let timestamp = get_current_timestamp()?;

        let signature =
            timestamp.clone() + self.api_key() + &recv_window.to_string() + &query_string;

        let signed_hex = hex_encode(encrypt_hmac_sha256(&self.secret_key, &signature)?);

        let result = self
            .http_client
            .get(
                "/v5/market/orderbook",
                Some(&[
                    ("category".to_string(), "spot".to_string()),
                    ("symbol".to_string(), base.to_string() + quote),
                    ("limit".to_string(), order_book_limit.to_string()),
                ]),
                Some(&[
                    ("X-BAPI-API-KEY".to_string(), self.api_key().to_string()),
                    ("X-BAPI-RECV-WINDOW".to_string(), recv_window.to_string()),
                    ("X-BAPI-TIMESTAMP".to_string(), timestamp),
                    ("X-BAPI-SIGN".to_string(), signed_hex),
                ]),
            )
            .await?;

        if result["retMsg"] != "OK" {
            return Err(format!("Fail fetch {} order book", self.name()).into());
        }

        let raw_asks = &result["result"]["a"];
        let raw_bids = &result["result"]["b"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }

        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
impl ExchangeStatic for Bybit {
    fn api_key(&self) -> &str {
        &self.api_key
    }
    fn secret_key(&self) -> &str {
        &self.secret_key
    }
    fn base_url(&self) -> &str {
        &self.base_url
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn is_pair_excluded(&self, quote: &str, base: &str) -> bool {
        self.excluded_trading_pairs.contains(&format!(
            "{}_{}",
            quote.to_uppercase(),
            base.to_uppercase()
        ))
    }
}
