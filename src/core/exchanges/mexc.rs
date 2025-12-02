use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use async_trait::async_trait;

use crate::{
    config::mexc::FETCH_NETWORKS,
    core::{
        net::{
            http::HttpClient,
            websocket::{BinaryMessageHandler, WebSocketClient},
        },
        traits::{ExchangeService, ExchangeStatic},
        types::{
            Asks, Bids, OrderBook, TradingPairs,
            structs::{Network, PriceData, TradingPair},
        },
        utils::{
            encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp, hex_encode,
            parse_json_as_bool, parse_string_typed_glass,
        },
    },
};

pub struct Mexc {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
    websocket_client: WebSocketClient,
}

impl Mexc {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        websocket_url: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // Создаем обработчик для protobuf сообщений MEXC
        let binary_handler: BinaryMessageHandler = Arc::new(|data: prost::bytes::Bytes| {
            match crate::config::mexc_protocol_buffers::decode_message(&data) {
                Ok(message) => {
                    let tickers =
                        crate::config::mexc_protocol_buffers::handle_protobuf_message(message);

                    // Преобразуем тикеры в строку для состояния
                    if !tickers.is_empty() {
                        let ticker_strings: Vec<String> = tickers
                            .iter()
                            .map(|t| format!("{}:{}", t.symbol, t.price))
                            .collect();
                        Some(ticker_strings.join("|"))
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Ошибка декодирования protobuf MEXC: {}", e);
                    None
                }
            }
        });
        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
            websocket_client: WebSocketClient::new(websocket_url)
                .with_ping_interval(Duration::from_secs(20), r#"{"method": "PING"}"#.to_string())
                .with_binary_handler(binary_handler),
        })
    }
    pub fn get_signature(
        &self,
        query_string: String,
        body_string: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let body_string = match body_string {
            Some(str) => str,
            None => "".to_string(),
        };

        let signature = query_string.to_string() + &body_string;
        let signed_hex = hex_encode(encrypt_hmac_sha256(&self.secret_key, &signature)?);

        Ok(signed_hex)
    }
}

impl ExchangeStatic for Mexc {
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
impl ExchangeService for Mexc {
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn Error>> {
        let recv_window = 5000;
        let timestamp = get_current_timestamp()?;
        let query_string = format!("recvWindow={}&timestamp={}", recv_window, timestamp);
        let sign = self.get_signature(query_string, None)?;

        let response = self
            .http_client
            .get(
                FETCH_NETWORKS,
                Some(&[
                    ("recvWindow".to_string(), recv_window.to_string()),
                    ("timestamp".to_string(), timestamp),
                    ("signature".to_string(), sign),
                ]),
                Some(&[
                    ("X-MEXC-APIKEY".to_string(), self.api_key().to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
            )
            .await?;

        let mut fetched_networks: Vec<Network> = Vec::new();

        for item in response.members() {
            if item["coin"] != coin {
                continue;
            }
            let networks = find_value_from_json_key(item, &["networkList"])?;
            for network in networks.members() {
                let withdraw_enabled = parse_json_as_bool(&network["withdrawEnable"])?;
                if !withdraw_enabled {
                    continue;
                }

                if let Ok(network) = Network::parse_json(
                    &network["netWork"],
                    &network["netWork"],
                    coin.to_string(),
                    Some(&network["withdrawFee"]),
                    &network["contract"],
                    None,
                    None,
                ) {
                    fetched_networks.push(network);
                };
            }
        }
        Ok(fetched_networks)
    }

    // Mexc close their margin interface
    async fn is_margin_available(&self, pair: &TradingPair) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn Error>> {
        // Просто запускаем, игнорируем ошибки соединения
        match self.websocket_client.get_state().await {
            None => {
                let client = self.websocket_client.clone();
                tokio::spawn(async move {
                    client
                        .run_with_reconnect(Some(
                            r#"{
                                "method": "SUBSCRIPTION",
                                "params": ["spot@public.miniTickers.v3.api.pb@UTC+0"]}"#,
                        ))
                        .await;
                });
                Err(format!("Subscribe to {} ticker updates", self.name).into())
            }
            Some(state) => {
                let mut trading_pairs: TradingPairs = HashMap::new();

                for ticker_info in state.split('|') {
                    if let Some((symbol, price_str)) = ticker_info.split_once(':') {
                        let Some(trading_pair) = TradingPair::from_str(symbol) else {
                            continue;
                        };
                        let Ok(price) = price_str.parse::<f64>() else {
                            continue;
                        };

                        trading_pairs.insert(
                            trading_pair,
                            PriceData::new((self.name.clone(), price), (self.name.clone(), price)),
                        );
                    }
                }

                return Ok(trading_pairs);
            }
        }
    }
    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let result = self
            .http_client
            .get(
                "/api/v3/depth",
                Some(&[
                    ("symbol".to_string(), base.to_string() + quote),
                    ("limit".to_string(), "500".to_string()),
                ]),
                Some(&[
                    ("X-MEXC-APIKEY".to_string(), self.api_key().to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
            )
            .await?;

        if !result.has_key("asks") || !result.has_key("bids") {
            return Err("Invalid response: missing 'asks' or 'bids' field".into());
        }
        let raw_asks = find_value_from_json_key(&result, &["asks"])?;
        let raw_bids = find_value_from_json_key(&result, &["bids"])?;

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }

        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
