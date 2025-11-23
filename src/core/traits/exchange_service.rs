use crate::core::types::{OrderBook, TradingPairs, structs::Network};
use async_trait::async_trait;

pub trait ExchangeStatic {
    fn api_key(&self) -> &str;
    fn secret_key(&self) -> &str;
    fn base_url(&self) -> &str;
    fn name(&self) -> &str;
}

#[async_trait]
pub trait ExchangeService {
    async fn fetch_orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>>;
    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn std::error::Error>>;
    async fn is_margin_available(&self, asset: &str) -> Result<bool, Box<dyn std::error::Error>>;
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>>;
}

pub trait Exchange: ExchangeStatic + ExchangeService + Send + Sync {}
impl<T> Exchange for T where T: ExchangeStatic + ExchangeService + Send + Sync {}
