pub trait Tradeble {
    fn base(&self) -> &str;
    fn quote(&self) -> &str;

    fn min_buy_price(&self) -> Option<(usize, f64)>;
    fn max_sell_price(&self) -> Option<(usize, f64)>;

    fn update_buy_price(&self, exchange_index: usize, price: f64);
    fn update_sell_price(&self, exchange_index: usize, price: f64);
}
