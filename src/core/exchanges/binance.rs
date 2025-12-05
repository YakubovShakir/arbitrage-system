use async_trait::async_trait;

use crate::{
    config,
    core::{
        net::{http::HttpClient, websocket::WebSocketClient},
        traits::{ExchangeService, ExchangeStatic},
        types::{Asks, Bids, Network, OrderBook, PriceData, TradingPair, TradingPairs},
        utils::{
            encrypt_hmac_sha256, get_current_timestamp, hex_encode, parse_json_as_bool,
            parse_string_typed_glass,
        },
    },
};
use core::{f64, str};
use std::{collections::HashMap, error::Error};

pub struct Binance {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
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
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
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
                    ("coin", coin),
                    ("timestamp", &timestamp),
                    ("signature", &hex_encode(signature)),
                ]),
                Some(&[("X-MBX-APIKEY", self.api_key())]),
            )
            .await?;

        let mut fetched_networks: Vec<Network> = Vec::new();

        for item in response.members() {
            if item["coin"] != coin {
                continue;
            }
            let deposit_enabled = parse_json_as_bool(&item["depositAllEnable"])?;
            let withdraw_enabled = parse_json_as_bool(&item["withdrawAllEnable"])?;
            if !deposit_enabled || !withdraw_enabled {
                break;
            }

            let networks = &item["networkList"];
            for network in networks.members() {
                if let Ok(network) = Network::parse_json(
                    &network["network"],
                    &network["name"],
                    coin.to_string(),
                    Some(&network["withdrawFee"]),
                    &network["contractAddress"],
                    None,
                    None,
                ) {
                    fetched_networks.push(network);
                }
            }
        }

        Ok(fetched_networks)
    }

    async fn is_margin_available(&self, pair: &TradingPair) -> Result<bool, Box<dyn Error>> {
        let response = self
            .http_client
            .get(
                config::binance::FETCH_MARGIN_INFO,
                Some(&[("asset", &pair.base)]),
                Some(&[("X-MBX-APIKEY", self.api_key())]),
            )
            .await?;

        Ok(response[0]["isBorrowable"].as_bool().ok_or(format!(
            "Не удалось распарсить ответ is_margin_available с биржи {} в bool-тип",
            self.name
        ))?)
    }

    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn Error>> {
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
                Err(format!("Subscribe to {} ticker updates", self.name).into())
            }
            Some(state) => {
                let parsed_tickers = json::parse(&state).or(Err(format!(
                    "Cannot parse ticker response from {} to JSON",
                    self.name
                )))?;

                let mut trading_pairs: TradingPairs = HashMap::new();
                for ticker in parsed_tickers.members() {
                    let symbol = &ticker["s"];
                    let last_price = &ticker["c"];
                    if let Some(trading_pair) = TradingPair::from_str(&symbol.to_string()) {
                        if let Ok(price) = last_price
                            .as_str()
                            .ok_or("Could not parse price as str")?
                            .parse::<f64>()
                        {
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
                return Ok(trading_pairs);
            }
        }
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let orderbook = self
            .http_client
            .get(
                "/api/v3/depth",
                Some(&[
                    ("symbol", format!("{}{}", base, quote).as_str()),
                    ("limit", "500"),
                ]),
                Some(&[("X-MBX-APIKEY", self.api_key())]),
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
