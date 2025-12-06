use async_trait::async_trait;
use hex_literal::hex;
use md5::{Digest, Md5};
use rand::{Rng, distributions::Alphanumeric, prelude::Distribution};
use uuid::Uuid;

use crate::core::{
    net::{
        http::HttpClient,
        websocket::{PingPongHandler, WebSocketClient},
    },
    traits::{ExchangeService, ExchangeStatic},
    types::{Asks, Bids, Network, OrderBook, PriceData, TradingPair, TradingPairs},
    utils::{
        base64_encode, encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp,
        hex_encode, parse_json_as_f64, parse_string_typed_glass, sign_rsa_sha256,
    },
};
use std::{collections::HashMap, error::Error, sync::Arc};

pub fn uuid_spot() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

pub struct Lbank {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
    // websocket_client: WebSocketClient,
}

impl Lbank {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        // websocket_url: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // let ping_handler: PingPongHandler = Arc::new(|text: &str| -> Option<String> {
        //     // Парсим JSON
        //     let json_data = match json::parse(text) {
        //         Ok(data) => data,
        //         Err(_) => return None, // Не JSON или не наш формат
        //     };

        //     // Проверяем, это ping от сервера?
        //     if json_data["action"].as_str() == Some("ping") {
        //         // Извлекаем UUID из ping
        //         if let Some(ping_id) = json_data["ping"].as_str() {
        //             // Создаем ответный pong
        //             let pong_response = json::object! {
        //                 "action": "pong",
        //                 "pong": ping_id
        //             };
        //             return Some(pong_response.dump());
        //         }
        //     }

        //     None // Не ping сообщение, оставляем как есть
        // });

        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
            // websocket_client: WebSocketClient::new(websocket_url), // .with_ping_pong_handler(ping_handler),
        })
    }

    fn get_sign(
        &self,
        echo_str: &str,
        timestamp: &str,
        // Дополнительные параметры метода
        extra_params: Option<&[(&str, &str)]>,
    ) -> Result<String, Box<dyn Error>> {
        // 1. Собираем все параметры в вектор
        let mut params = vec![
            ("api_key", self.api_key.as_str()),
            ("echostr", echo_str),
            ("signature_method", "RSA"),
            ("timestamp", timestamp),
        ];
        // 2. Добавляем дополнительные параметры если есть
        if let Some(extra) = extra_params {
            params.extend(extra);
        }

        // 3. Сортируем по имени параметра
        params.sort_by(|a, b| a.0.cmp(b.0));

        // 4. Собираем строку
        let parameters = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        // 5. MD5
        let mut hasher = Md5::new();
        hasher.update(parameters);
        let prepared_str_hex = hex::encode(hasher.finalize()).to_uppercase();

        // 6. HMAC-SHA256 + Base64
        let encrypted = sign_rsa_sha256(&self.secret_key, &prepared_str_hex)?;
        let encoded = base64_encode(&encrypted);

        Ok(encoded)
    }

    pub fn get_auth_headers(
        &self,
        echo_str: String,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        Ok(vec![
            (
                "contentType".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
            ("timestamp".to_string(), get_current_timestamp()?),
            ("signature_method".to_string(), "RSA".to_string()),
            ("echostr".to_string(), echo_str),
        ])
    }
}

impl ExchangeStatic for Lbank {
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
impl ExchangeService for Lbank {
    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .get("/v2/supplement/ticker/price.do", None, None)
            .await?;

        let tickers = find_value_from_json_key(&response, &["data"])?;
        if tickers.is_empty() {
            return Err(format!("{} Invalid response: 'tickers' is empty", self.name).into());
        }

        let mut trading_pairs: TradingPairs = HashMap::new();

        for ticker in tickers.members() {
            let Some(pair) =
                TradingPair::from_str_with_separator(&ticker["symbol"].to_string(), '_')
            else {
                continue;
            };

            let Ok(price) = parse_json_as_f64(&ticker["price"]) else {
                continue;
            };

            let price_data = PriceData::new((self.name.clone(), price), (self.name.clone(), price));

            trading_pairs.insert(pair, price_data);
        }
        Ok(trading_pairs)
    }

    async fn is_margin_available(
        &self,
        trading_pair_name: &TradingPair,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(false)
    }
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let timestamp_response = self.http_client.get("/v2/timestamp.do", None, None).await?;

        let timestamp = timestamp_response["data"].to_string();
        let echo_str = uuid_spot();

        let headers = vec![
            (
                "contentType".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
            ("timestamp".to_string(), timestamp.clone()),
            ("signature_method".to_string(), "RSA".to_string()),
            ("echostr".to_string(), echo_str.clone()),
        ];

        // Для user_info.do нет дополнительных параметров
        let sign = self.get_sign(&echo_str, &timestamp, None)?;
        println!("Generated sign: {}", sign);

        // Параметры в ТЕЛЕ запроса
        let body_params = vec![
            ("api_key".to_string(), self.api_key.clone()),
            ("sign".to_string(), sign),
        ];

        println!("Sending to LBank:");
        println!("Headers: {:?}", headers);
        println!("Body params: {:?}", body_params);

        let response = self
            .http_client
            .post(
                "/v2/supplement/user_info.do",
                None,
                Some(&headers),
                Some(&body_params),
            )
            .await?;

        println!("RESPONSE: {}", response.pretty(2));

        Ok(vec![])
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let timestamp = get_current_timestamp()?;
        let response = self
            .http_client
            .get(
                "/v2/depth.do",
                Some(&[
                    (
                        "symbol".to_string(),
                        format!(
                            "{}_{}",
                            base.to_string().to_lowercase(),
                            &quote.to_string().to_lowercase()
                        ),
                    ),
                    ("size".to_string(), "200".to_string()),
                ]),
                Some(&[
                    (
                        "Content-Type".to_string(),
                        "application/x-www-form-urlencoded".to_string(),
                    ),
                    ("timestamp".to_string(), timestamp),
                ]),
            )
            .await?;

        if response["msg"] != "Success" {
            return Err(format!(
                "Invalid response {}: missing 'asks' or 'bids' field - {}",
                self.name(),
                response
            )
            .into());
        }

        let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
        let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }
        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
