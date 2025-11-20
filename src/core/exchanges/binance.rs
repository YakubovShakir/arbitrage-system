use async_trait::async_trait;

use crate::{
    config,
    core::{
        net::{http::HttpClient, websocket::WebSocketClient},
        traits::{ExchangeService, ExchangeStatic},
        types::{
            Asks, Bids, OrderBook, TradingPairs,
            structs::{Network, PriceData, TradingPair},
        },
        utils::{encrypt_hmac_sha256, get_current_timestamp, hex_encode, parse_string_typed_glass},
    },
};
use core::{f64, str};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Debug)]
pub struct Binance {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    excluded_trading_pairs: HashSet<String>,
    http_client: HttpClient,
    websocket_client: WebSocketClient,
}

impl Binance {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        websocket_url: &str,
        excluded_trading_pairs: &[&str],
    ) -> Result<Self, Box<dyn Error>> {
        let mut set: HashSet<String> = HashSet::new();
        for excluded_pair in excluded_trading_pairs {
            set.insert(excluded_pair.to_string().to_uppercase());
        }

        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            excluded_trading_pairs: set,
            http_client: HttpClient::new(base_url)?,
            websocket_client: WebSocketClient::new(websocket_url),
        })
    }
}

impl ExchangeStatic for Binance {
    fn name(&self) -> &str {
        &self.name
    }
    fn api_key(&self) -> &str {
        &self.api_key
    }
    fn secret_key(&self) -> &str {
        &self.secret_key
    }
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn is_pair_excluded(&self, quote: &str, base: &str) -> bool {
        self.excluded_trading_pairs.contains(&format!(
            "{}_{}",
            quote.to_uppercase(),
            base.to_uppercase()
        ))
    }
}

#[async_trait]
impl ExchangeService for Binance {
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let timestamp = get_current_timestamp()?;
        let query_string = format!("coin={}&timestamp={}", coin, timestamp);
        let signature = encrypt_hmac_sha256(&self.secret_key, &query_string)?;

        let response = self
            .http_client
            .get(
                config::binance::FETCH_NETWORKS,
                Some(&[
                    ("coin".to_string(), coin.to_string()),
                    ("timestamp".to_string(), timestamp),
                    ("signature".to_string(), hex_encode(signature)),
                ]),
                Some(&[("X-MBX-APIKEY".to_string(), self.api_key().to_string())]),
            )
            .await?;
        let mut fetched_networks: Vec<Network> = Vec::new();

        for item in response.members() {
            if !item.has_key("coin") {
                continue;
            }

            if item["coin"] != coin {
                continue;
            }

            if !item.has_key("networkList") {
                return Err(format!(
                    "Binance not found [networkList] property in response for {}",
                    coin
                )
                .into());
            }

            let (Some(deposit_enabled), Some(withdraw_enabled)) = (
                item["depositAllEnable"].as_bool(),
                item["withdrawAllEnable"].as_bool(),
            ) else {
                break;
            };

            if !deposit_enabled || !withdraw_enabled {
                break;
            }

            let networks = &item["networkList"];

            for network in networks.members() {
                let network_name = &network["network"];
                let coin_name = &network["coin"];
                let full_name = &network["name"];
                let contract_address = if network.has_key("contractAddress") {
                    Some(network["contractAddress"].to_string())
                } else {
                    None
                };

                let parsed_network: Network = Network::new(
                    network_name.to_string(),
                    full_name.to_string(),
                    coin_name.to_string(),
                    contract_address,
                    None,
                    None,
                );
                fetched_networks.push(parsed_network);
            }
        }
        // println!("Network response is {:?}", response[0]["coin"]);

        Ok(fetched_networks)
    }

    async fn is_margin_available(&self, asset: &str) -> Result<bool, Box<dyn Error>> {
        let response = self
            .http_client
            .get(
                config::binance::FETCH_MARGIN_INFO,
                Some(&[("asset".to_string(), asset.to_string())]),
                Some(&[("X-MBX-APIKEY".to_string(), self.api_key().to_string())]),
            )
            .await?;

        Ok(response[0]["isBorrowable"].as_bool().ok_or(format!(
            "Не удалось распарсить ответ is_margin_available с биржи {} в bool-тип",
            self.name
        ))?)
    }

    async fn fetch_tickers(&self) -> Option<TradingPairs> {
        // Просто запускаем, игнорируем ошибки соединения
        match self.websocket_client.get_state().await {
            None => {
                let client = self.websocket_client.clone();
                tokio::spawn(async move {
                    client
                        .run_with_reconnect(Some(
                            r#"{"method":"SUBSCRIBE","params":["!ticker@arr"],"id":1}"#,
                        ))
                        .await;
                });
                None
            }
            Some(state) => {
                let parsed_tickers =
                    json::parse(&state).expect("Cannot parse ticker response from BINANCE to JSON");
                let mut trading_pairs: TradingPairs = HashMap::new();
                for ticker in parsed_tickers.members() {
                    let symbol = &ticker["s"];
                    let last_price = &ticker["c"];
                    if let Some(trading_pair) = TradingPair::from_str(&symbol.to_string()) {
                        if let Ok(price) = last_price.as_str()?.parse::<f64>() {
                            trading_pairs.insert(
                                trading_pair,
                                PriceData::new(
                                    (self.name.clone(), price),
                                    (self.name.clone(), price),
                                ),
                            );
                        }
                    }
                }
                return Some(trading_pairs);
            }
        }
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let orderbook = self
            .http_client
            .get(
                "/api/v3/depth",
                Some(&[
                    ("symbol".to_string(), format!("{}{}", base, quote)),
                    ("limit".to_string(), "500".to_string()),
                ]),
                Some(&[("X-MBX-APIKEY".to_string(), self.api_key().to_string())]),
            )
            .await?;

        if !orderbook.has_key("asks") || !orderbook.has_key("bids") {
            return Err("Invalid response: missing 'asks' or 'bids' field".into());
        }

        let raw_asks = &orderbook["asks"];
        let raw_bids = &orderbook["bids"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }

        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
