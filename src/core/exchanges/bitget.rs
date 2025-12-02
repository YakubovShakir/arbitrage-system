use async_trait::async_trait;
use futures_util::SinkExt;
use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    config,
    core::{
        net::{
            http::HttpClient,
            websocket::{ConnectionHandler, WebSocketClient},
        },
        traits::{ExchangeService, ExchangeStatic},
        types::{
            Asks, Bids, OrderBook, Price, TradingPairs,
            structs::{Network, PriceData, TradingPair},
        },
        utils::{
            base64_encode, encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp,
            parse_json_as_bool, parse_json_as_f64, parse_string_typed_glass,
        },
    },
};

pub struct Bitget {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
    // websocket_client: WebSocketClient,
}

impl Bitget {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        // websocket_url: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // // Клонируем ВСЕ данные для использования в замыкании
        // let name = name.to_string();
        // let api_key = api_key.to_string();
        // let secret_key = secret_key.to_string();
        // let websocket_url = websocket_url.to_string();
        // let passphrase = config::bitget::PASSPHRASE.to_string();

        // let handler: ConnectionHandler = Arc::new({
        //     let name = name.clone();
        //     let api_key = api_key.clone();
        //     let secret_key = secret_key.clone();
        //     let websocket_url = websocket_url.clone();
        //     let passphrase = passphrase.clone();

        //     move || {
        //         // Клонируем для внутреннего замыкания
        //         let name = name.clone();
        //         let api_key = api_key.clone();
        //         let secret_key = secret_key.clone();
        //         let websocket_url = websocket_url.clone();
        //         let passphrase = passphrase.clone();

        //         Box::pin(async move {
        //             // Каждый раз генерируем новый timestamp и signature
        //             let timestamp = match get_current_timestamp() {
        //                 Ok(time) => time,
        //                 Err(e) => {
        //                     return Err(
        //                         format!("{} error in ConnectionHandler - {}", name, e).into()
        //                     );
        //                 }
        //             };

        //             let signature = timestamp.clone() + "GET" + "/user/verify";
        //             let encrypted = match encrypt_hmac_sha256(&secret_key, &signature) {
        //                 Ok(encrypted) => encrypted,
        //                 Err(e) => {
        //                     return Err(
        //                         format!("{} error in ConnectionHandler - {}", name, e).into()
        //                     );
        //                 }
        //             };

        //             let encoded_sign = base64_encode(&encrypted);

        //             let login_message = format!(
        //                 r#"{{
        //                 "op":"login",
        //                 "args":[
        //                     {{
        //                     "apiKey":"{}",
        //                     "passphrase":"{}",
        //                     "timestamp":"{}",
        //                     "sign":"{}"
        //                     }}
        //                 ]
        //                 }}"#,
        //                 api_key, passphrase, timestamp, encoded_sign
        //             );

        //             let (mut ws_stream, response) = connect_async(&websocket_url).await?;
        //             println!("WebSocket соединение c {} установлено", websocket_url);

        //             ws_stream
        //                 .send(Message::Text(login_message.into()))
        //                 .await
        //                 .map_err(|e| format!("Failed to send login message to Bitget: {}", e))?;
        //             println!("Отправили login message");
        //             // Возвращаем целый ws_stream
        //             Ok((ws_stream, response))
        //         })
        //     }
        // });

        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
            // websocket_client: WebSocketClient::new(&websocket_url)
            //     // .with_ping_interval(Duration::from_secs(20), r#"ping"#.to_string())
            //     .with_connection_handler(handler),
        })
    }
    pub fn get_sign(
        &self,
        method: &str,
        endpoint: &str,
        query: &str,
    ) -> Result<String, Box<dyn Error>> {
        let signature = get_current_timestamp()? + method + endpoint + "?" + query;
        Ok(base64_encode(&encrypt_hmac_sha256(
            &self.secret_key,
            &signature,
        )?))
    }
}

#[async_trait]
impl ExchangeStatic for Bitget {
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
impl ExchangeService for Bitget {
    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn std::error::Error>> {
        // Просто запускаем, игнорируем ошибки соединения
        let response = self
            .http_client
            .get("/api/v2/spot/market/tickers", None, None)
            .await?;

        let data = find_value_from_json_key(&response, &["data"])?;
        if !data.is_array() {
            return Err(format!("{} Invalid response: 'data' is not an array", self.name).into());
        }
        let mut trading_pairs: TradingPairs = HashMap::new();

        for ticker in data.members() {
            let Some(trading_pair) = TradingPair::from_str(&ticker["symbol"].to_string()) else {
                continue;
            };

            let (Ok(ask_price), Ok(bid_price)) = (
                parse_json_as_f64(&ticker["askPr"]),
                parse_json_as_f64(&ticker["bidPr"]),
            ) else {
                continue;
            };

            trading_pairs.insert(
                trading_pair,
                PriceData::new(
                    (self.name.clone(), bid_price),
                    (self.name.clone(), ask_price),
                ),
            );
        }

        Ok(trading_pairs)
    }

    async fn is_margin_available(
        &self,
        trading_pair_name: &TradingPair,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sign = self.get_sign("GET", "/api/v2/margin/currencies", "")?;
        let response = self
            .http_client
            .get(
                "/api/v2/margin/currencies",
                None,
                Some(&[
                    ("ACCESS-KEY".to_string(), self.api_key.to_string()),
                    ("ACCESS-SIGN".to_string(), sign),
                ]),
            )
            .await?;
        let data = find_value_from_json_key(&response, &["data"])?;
        if !data.is_array() {
            return Err(format!("{} Invalid response: 'data'is not an array", self.name).into());
        }

        for item in data.members() {
            if item["baseCoin"] != trading_pair_name.base {
                continue;
            }
            let borrowable = parse_json_as_bool(&item["isBorrowable"])?;
            return Ok(borrowable);
        }
        Ok(false)
    }

    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .get(
                "/api/v2/spot/public/coins",
                Some(&[("coin".to_string(), coin.to_string())]),
                None,
            )
            .await?;

        let data = find_value_from_json_key(&response, &["data"])?;

        if !data.is_array() {
            return Err(format!("{} Invalid response: 'data' is not an array", self.name).into());
        }
        if data.is_empty() {
            return Err(format!("{} Invalid response: 'data' is empty array", self.name).into());
        }

        let chains = find_value_from_json_key(&data[0], &["chains"])?;

        if !chains.is_array() {
            return Err(format!("{} Invalid response: 'chains' is not an array", self.name).into());
        }
        let mut fetched_networks: Vec<Network> = Vec::new();

        for chain in chains.members() {
            if let (Ok(withrawable), Ok(rechargeable)) = (
                parse_json_as_bool(&chain["withdrawable"]),
                parse_json_as_bool(&chain["rechargeable"]),
            ) {
                if !withrawable || !rechargeable {
                    continue;
                }
            } else {
                continue;
            };

            if let Ok(network) = Network::parse_json(
                &chain["chain"],
                &chain["chain"],
                coin.to_string(),
                Some(&chain["withdrawFee"]),
                &chain["contractAddress"],
                None,
                None,
            ) {
                fetched_networks.push(network);
            }
        }
        Ok(fetched_networks)
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let result = self
            .http_client
            .get(
                "/api/v2/spot/market/orderbook",
                Some(&[
                    (
                        "symbol".to_string(),
                        (base.to_string() + quote).to_lowercase(),
                    ),
                    ("limit".to_string(), "150".to_string()),
                    ("type".to_string(), "step0".to_string()),
                ]),
                Some(&[("Content-Type".to_string(), "application/json".to_string())]),
            )
            .await?;

        // .await?;
        if result["msg"] != "success" {
            return Err(format!("Fail fetch {} order book", self.name()).into());
        }

        let raw_asks = &result["data"]["asks"];
        let raw_bids = &result["data"]["bids"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }
        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
