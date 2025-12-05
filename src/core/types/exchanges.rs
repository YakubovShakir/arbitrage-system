use crate::core::net::{http::HttpClient, websocket::WebSocketClient};

pub struct ExchangeConfig {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
    websocket_client: Option<WebSocketClient>,
}

pub enum Exchange {
    Binance(ExchangeConfig),
    Bybit(ExchangeConfig),
    Bitget(ExchangeConfig),
    Gate(ExchangeConfig),
    Kucoin(ExchangeConfig),
    Mexc(ExchangeConfig),
}
impl Exchange {
    pub fn config(&self) -> &ExchangeConfig {
        match self {
            Exchange::Binance(config) => config,
            Exchange::Bybit(config) => config,
            Exchange::Bitget(config) => config,
            Exchange::Gate(config) => config,
            Exchange::Kucoin(config) => config,
            Exchange::Mexc(config) => config,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.config().name
    }
    pub fn get_api_key(&self) -> &str {
        &self.config().api_key
    }
    pub fn get_secret_key(&self) -> &str {
        &self.config().secret_key
    }
    pub fn get_base_url(&self) -> &str {
        &self.config().base_url
    }
    pub fn get_http_client(&self) -> &HttpClient {
        &self.config().http_client
    }
    pub fn get_socket_client(&self) -> &Option<WebSocketClient> {
        &self.config().websocket_client
    }
}
