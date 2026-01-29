use crate::core::types::{ExchangeNetworks, OrderBook, Tickers, TradingPair};
use async_trait::async_trait;

#[async_trait]
pub trait TickerService {
    async fn tickers(&self) -> Result<Tickers, Box<dyn std::error::Error>>;
}
#[async_trait]
pub trait OrderBookService {
    async fn orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait NetworkService {
    async fn networks(&self, coin: &str) -> Result<ExchangeNetworks, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait MarginInfoService {
    async fn borrowable(
        &self,
        trading_pair_name: &TradingPair,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
