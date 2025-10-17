use async_trait::async_trait;

use crate::core::OrderBook;

#[async_trait]
pub trait ExchangeAPI: Send + Sync {
    fn api_key(&self) -> &str;
    fn secret_key(&self) -> &str;
    fn base_url(&self) -> &str;
    fn name(&self) -> &str;

    fn is_pair_available(&self, pair: String) -> bool;
    async fn fetch_order_book(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>>;
}
