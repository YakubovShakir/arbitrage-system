use async_trait::async_trait;

use crate::core::{
    net::{http::HttpClient, websocket::WebSocketClient},
    traits::exchange_service::{Exchange, ExchangeService, ExchangeStatic},
    types::{Asks, Bids, OrderBook},
    utils::parse_string_typed_glass,
};
use std::{collections::HashSet, error::Error};

#[derive(Debug)]
pub struct Binance {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    excluded_trading_pairs: HashSet<String>,
    http_client: HttpClient,
    ws_client: WebSocketClient,
}

impl Binance {
    pub async fn new(
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
        let mut ws_client = WebSocketClient::new(websocket_url);
        let _ = ws_client.connect().await;

        let subscribe_msg = r#"{"method":"SUBSCRIBE","params":["!ticker@arr"],"id":1}"#;
        ws_client.send_message(subscribe_msg).await;

        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            excluded_trading_pairs: set,
            http_client: HttpClient::new(base_url)?,
            ws_client,
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
    fn tickets(&self) -> &str {
        &self.tickets
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
    async fn fetch_tickets(&self) {
        match self.ws_client.read_message().await {
            Some(tickets) => {
                println!("{}", tickets)
            }
            None => (),
        };
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
