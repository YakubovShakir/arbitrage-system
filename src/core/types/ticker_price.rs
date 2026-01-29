use super::{ExchangeName, Price};

#[derive(Clone)]
pub struct TickerPrice {
    pub buy_price: (ExchangeName, Price),
    pub sell_price: (ExchangeName, Price),
}
