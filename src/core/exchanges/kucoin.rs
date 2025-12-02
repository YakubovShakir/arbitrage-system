use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;

use crate::{
    config,
    core::{
        net::{
            http::HttpClient,
            websocket::{ConnectionHandler, WebSocketClient},
        },
        traits::{ExchangeService, ExchangeStatic},
        types::{
            Asks, Bids, OrderBook, TradingPairs,
            structs::{Network, PriceData, TradingPair},
        },
        utils::{
            base64_encode, encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp,
            parse_json_as_bool, parse_string_typed_glass,
        },
    },
};
use std::{collections::HashMap, sync::Arc, time::Duration};

pub struct Kucoin {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
    websocket_client: WebSocketClient,
}

impl Kucoin {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        websocket_url: &str,
        base_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = HttpClient::new(base_url)?;

        let handler: ConnectionHandler = Arc::new({
            let http_client = http_client.clone();
            let name = name.to_string();

            move || {
                let http_client = http_client.clone();
                let name = name.clone();

                Box::pin(async move {
                    // Вся логика подключения
                    let response = match http_client
                        .post("/api/v1/bullet-public", None, None, None)
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            return Err(
                                format!("{} error in ConnectionHandler - {}", name, e).into()
                            );
                        }
                    };

                    if response["code"] != "200000" {
                        return Err(format!("API error from {}", name).into());
                    }

                    let token = match find_value_from_json_key(&response, &["data", "token"]) {
                        Ok(token) => token,
                        Err(e) => {
                            return Err(
                                format!("{} error in ConnectionHandler - {}", name, e).into()
                            );
                        }
                    };

                    let servers =
                        match find_value_from_json_key(&response, &["data", "instanceServers"]) {
                            Ok(servers) => servers,
                            Err(e) => {
                                return Err(
                                    format!("{} error in ConnectionHandler - {}", name, e).into()
                                );
                            }
                        };

                    let endpoint = match find_value_from_json_key(&servers[0], &["endpoint"]) {
                        Ok(endpoint) => endpoint,
                        Err(e) => {
                            return Err(
                                format!("{} error in ConnectionHandler - {}", name, e).into()
                            );
                        }
                    };

                    let url = format!("{}?token={}", endpoint, token);

                    let result = connect_async(&url).await?;
                    println!("WebSocket соединение c {} установлено", endpoint);

                    Ok(result)
                })
            }
        });

        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client,
            websocket_client: WebSocketClient::new(websocket_url)
                .with_ping_interval(
                    Duration::from_secs(20),
                    r#"{"id": "123","type": "ping"}"#.to_string(),
                )
                .with_connection_handler(handler) // ← просто передаем замыкание
                .with_streamed_state(),
        })
    }
    fn get_sign(
        &self,
        method: &str,
        endpoint: &str,
        query: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = get_current_timestamp()?;
        let sign = (&timestamp).to_string() + method + endpoint + "?" + query;
        Ok(base64_encode(&encrypt_hmac_sha256(
            &self.secret_key,
            &sign,
        )?))
    }
    fn get_auth_headers(
        &self,
        sign: String,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        // kc-api-timestamp
        let kc_api_timestamp = get_current_timestamp()?;

        // kc-api-passphrase
        let kc_api_passphrase = base64_encode(&encrypt_hmac_sha256(
            &self.secret_key,
            &config::kucoin::PASSPHRASE.to_string(),
        )?);

        //kc-api-version
        let kc_api_version = "3".to_string();
        Ok(vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("KC-API-KEY".to_string(), self.api_key.to_string()),
            ("KC-API-SIGN".to_string(), sign),
            ("KC-API-TIMESTAMP".to_string(), kc_api_timestamp),
            ("KC-API-PASSPHRASE".to_string(), kc_api_passphrase),
            ("KC-API-KEY-VERSION".to_string(), kc_api_version),
        ])
    }
}

impl ExchangeStatic for Kucoin {
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
impl ExchangeService for Kucoin {
    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn std::error::Error>> {
        match self.websocket_client.get_streamed_state().await {
            None => {
                let client = self.websocket_client.clone();
                tokio::spawn(async move {
                    client
                        .run_with_reconnect(Some(
                            r#"{
                                "id": 123,
                                "type": "subscribe",
                                "topic": "/market/ticker:all",
                                "response": true
                                }"#,
                        ))
                        .await;
                });
                Err(format!("Subscribe to {} ticker updates", self.name).into())
            }
            Some(vec) => {
                let mut trading_pairs: TradingPairs = HashMap::new();
                for item in vec {
                    let parsed_ticker = json::parse(&item).or(Err(format!(
                        "Cannot parse ticker  from {} to JSON",
                        self.name
                    )))?;
                    let Some(trading_pair) = TradingPair::from_str_with_separator(
                        &parsed_ticker["subject"].to_string(),
                        '-',
                    ) else {
                        continue;
                    };
                    let Ok(best_ask) = parsed_ticker["data"]["bestAsk"]
                        .as_str()
                        .ok_or("Could not parse price as str")?
                        .parse::<f64>()
                    else {
                        continue;
                    };
                    let Ok(best_bid) = parsed_ticker["data"]["bestBid"]
                        .as_str()
                        .ok_or("Could not parse price as str")?
                        .parse::<f64>()
                    else {
                        continue;
                    };

                    trading_pairs.insert(
                        trading_pair,
                        PriceData::new(
                            (self.name.clone(), best_bid),
                            (self.name.clone(), best_ask),
                        ),
                    );
                }

                return Ok(trading_pairs);
            }
        }
    }
    async fn is_margin_available(
        &self,
        pair: &TradingPair,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .get(
                "/api/v3/currencies/",
                Some(&[("currency".to_string(), pair.base.to_string())]),
                None,
            )
            .await?;

        let margin_enabled = parse_json_as_bool(&find_value_from_json_key(
            &response,
            &["data", "isMarginEnabled"],
        )?)?;

        Ok(margin_enabled)
    }

    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .get(&format!("/api/v3/currencies/{}", coin), None, None)
            .await?;

        let chains = find_value_from_json_key(&response, &["data", "chains"])?;
        let mut fetched_networks: Vec<Network> = Vec::new();

        for chain in chains.members() {
            if let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                parse_json_as_bool(&chain["isWithdrawEnabled"]),
                parse_json_as_bool(&chain["isDepositEnabled"]),
            ) {
                if !withdraw_enabled || !deposit_enabled {
                    continue;
                }
            } else {
                continue;
            };

            if let Ok(network) = Network::parse_json(
                &chain["chainId"],
                &chain["chainName"],
                coin.to_string(),
                Some(&chain["withdrawMinFee"]),
                &chain["contractAddress"],
                None,
                None,
            ) {
                fetched_networks.push(network);
            }
        }
        Ok(fetched_networks)
    }

    async fn fetch_orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>> {
        let query = format!("symbol={}-{}", base.to_uppercase(), quote.to_uppercase());
        let sign = self.get_sign("GET", "/api/v3/market/orderbook/level2", &query)?;
        let auth_headers = self.get_auth_headers(sign)?;
        let response = self
            .http_client
            .get(
                "/api/v3/market/orderbook/level2",
                Some(&[(
                    "symbol".to_string(),
                    format!("{}-{}", base.to_uppercase(), quote.to_uppercase()),
                )]),
                Some(&auth_headers),
            )
            .await?;

        if response["code"] != "200000" {
            return Err(format!("Fail fetch {} order book", self.name()).into());
        }

        let raw_asks = &response["data"]["asks"];
        let raw_bids = &response["data"]["bids"];
        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err(format!(
                "Invalid response: 'asks' or 'bids' is not an array - {}",
                response
            )
            .into());
        }

        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
