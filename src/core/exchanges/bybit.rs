use std::{collections::HashMap, error::Error};

use async_trait::async_trait;

use crate::{
    config,
    core::{
        net::http::HttpClient,
        traits::{ExchangeService, ExchangeStatic},
        types::{Asks, Bids, Network, OrderBook, PriceData, TradingPair, TradingPairs},
        utils::{
            encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp, hex_encode,
            parse_json_as_f64, parse_string_typed_glass,
        },
    },
};
#[derive(Debug)]
pub struct Bybit {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
}

impl Bybit {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
        })
    }

    pub fn get_auth_headers(
        &self,
        query_string: String,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let recv_window = 5000;
        let timestamp = get_current_timestamp()?;
        let signature =
            timestamp.clone() + self.api_key() + &recv_window.to_string() + &query_string;
        let signed_hex = hex_encode(encrypt_hmac_sha256(&self.secret_key, &signature)?);

        Ok(vec![
            ("X-BAPI-API-KEY".to_string(), self.api_key().to_string()),
            ("X-BAPI-RECV-WINDOW".to_string(), recv_window.to_string()),
            ("X-BAPI-TIMESTAMP".to_string(), timestamp),
            ("X-BAPI-SIGN".to_string(), signed_hex),
        ])
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
}

#[async_trait]
impl ExchangeService for Bybit {
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn Error>> {
        let query_string = format!("coin={}", coin);
        let auth_headers = self.get_auth_headers(query_string)?;

        let response = self
            .http_client
            .get(
                config::bybit::FETCH_NETWORKS,
                Some(&[("coin".to_string(), coin.to_string())]),
                Some(&auth_headers),
            )
            .await?;

        let rows = find_value_from_json_key(&response, &["result", "rows"])?;
        let first_item = rows
            .members()
            .nth(0)
            .ok_or("Could not parse rows first item")?;
        let networks = find_value_from_json_key(&first_item, &["chains"])?;

        let mut fetched_networks: Vec<Network> = Vec::new();

        for network in networks.members() {
            if network["chainDeposit"] != "1" || network["chainWithdraw"] != "1" {
                continue;
            }
            if let Ok(network) = Network::parse_json(
                &network["chain"],
                &network["chainType"],
                coin.to_string(),
                Some(&network["withdrawFee"]),
                &network["contractAddress"],
                None,
                None,
            ) {
                fetched_networks.push(network);
            }
        }

        Ok(fetched_networks)
    }

    async fn is_margin_available(&self, pair: &TradingPair) -> Result<bool, Box<dyn Error>> {
        let response = self
            .http_client
            .get(
                config::bybit::FETCH_MARGIN_INFO,
                Some(&[("currency".to_string(), pair.base.to_string())]),
                None,
            )
            .await?;

        let vip_level = 0;
        let vip_list = find_value_from_json_key(&response, &["result", "vipCoinList"])?;

        let vip_list_level = vip_list
            .members()
            .nth(vip_level)
            .ok_or("vipCoinList is is empty")?;

        let list_item = vip_list_level["list"]
            .members()
            .nth(0)
            .ok_or("list is empty")?;

        let borrowable = find_value_from_json_key(list_item, &["borrowable"])?
            .as_bool()
            .ok_or("Could not parse borrowable as bool")?;

        Ok(borrowable)
    }

    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn Error>> {
        let response = self
            .http_client
            .get(
                "/v5/market/tickers",
                Some(&[("category".to_string(), "spot".to_string())]),
                None,
            )
            .await?;

        let mut trading_pairs: TradingPairs = HashMap::new();

        let tickers = find_value_from_json_key(&response, &["result", "list"])?;

        for ticker in tickers.members() {
            let (Ok(parsed_ask_price), Ok(parsed_bid_price)) = (
                parse_json_as_f64(&ticker["ask1Price"]),
                parse_json_as_f64(&ticker["bid1Price"]),
            ) else {
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
        Ok(trading_pairs)
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let order_book_limit = 500;
        let query_string = format!(
            "category={}&symbol={}&limit={}",
            "spot",
            base.to_string() + quote,
            order_book_limit,
        );
        let auth_headers = self.get_auth_headers(query_string)?;

        let result = self
            .http_client
            .get(
                "/v5/market/orderbook",
                Some(&[
                    ("category".to_string(), "spot".to_string()),
                    ("symbol".to_string(), base.to_string() + quote),
                    ("limit".to_string(), order_book_limit.to_string()),
                ]),
                Some(&auth_headers),
            )
            .await?;

        let raw_asks = find_value_from_json_key(&result, &["result", "a"])?;
        let raw_bids = find_value_from_json_key(&result, &["result", "b"])?;

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }

        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
