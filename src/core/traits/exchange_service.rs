use crate::core::types::OrderBook;
use async_trait::async_trait;

pub trait ExchangeStatic {
    fn api_key(&self) -> &str;
    fn secret_key(&self) -> &str;
    fn base_url(&self) -> &str;
    fn name(&self) -> &str;
    fn tickets(&self) -> &str;

    fn is_pair_excluded(&self, base: &str, quote: &str) -> bool;
}

#[async_trait]
pub trait ExchangeRestService: Send + Sync {
    async fn fetch_orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>>;

    async fn fetch_tickets(&self);
}

pub trait ExchangeSocketService: Send + Sync {
    async fn subscribe_to_tickets_update(&self);
    async fn get_tickets(&self);
}

pub trait Exchange: ExchangeStatic + ExchangeRestService + ExchangeSocketService {}

impl<T> Exchange for T where T: ExchangeStatic + ExchangeRestService + ExchangeSocketService {}
