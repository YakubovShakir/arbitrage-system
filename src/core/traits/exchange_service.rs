use crate::core::types::{OrderBook, TradingPairs};
use async_trait::async_trait;

pub trait ExchangeStatic {
    fn api_key(&self) -> &str;
    fn secret_key(&self) -> &str;
    fn base_url(&self) -> &str;
    fn name(&self) -> &str;

    fn is_pair_excluded(&self, base: &str, quote: &str) -> bool;
}

#[async_trait]
pub trait ExchangeService {
    async fn fetch_orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>>;

    async fn fetch_tickers(&self) -> Option<TradingPairs>;

    async fn is_margin_available(&self, asset: &str) -> bool;
}

pub trait Exchange: ExchangeStatic + ExchangeService + Send + Sync {}

impl<T> Exchange for T where T: ExchangeStatic + ExchangeService + Send + Sync {}
