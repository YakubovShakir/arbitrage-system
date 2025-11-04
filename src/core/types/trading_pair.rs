use std::sync::RwLock;

use crate::core::traits::Tradeble;
pub struct TradingPair {
    base: String,
    quote: String,

    min_buy_price: RwLock<Option<(usize, f64)>>,
    max_sell_price: RwLock<Option<(usize, f64)>>,
}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> TradingPair {
        TradingPair {
            base: base.to_string().to_uppercase(),
            quote: quote.to_string().to_uppercase(),
            max_sell_price: RwLock::new(None),
            min_buy_price: RwLock::new(None),
        }
    }
}
impl Tradeble for TradingPair {
    fn base(&self) -> &str {
        &self.base
    }
    fn quote(&self) -> &str {
        &self.quote
    }
    fn min_buy_price(&self) -> Option<(usize, f64)> {
        match self.min_buy_price.read() {
            Ok(quard) => quard.clone(),
            Err(_) => None,
        }
    }
    fn max_sell_price(&self) -> Option<(usize, f64)> {
        match self.max_sell_price.read() {
            Ok(quard) => quard.clone(),
            Err(_) => None,
        }
    }

    fn update_buy_price(&self, exchange_index: usize, price: f64) {
        match self.min_buy_price.write() {
            Ok(mut quard) => match &*quard {
                None => *quard = Some((exchange_index, price)),
                Some((current_index, current_min))
                    if price < *current_min
                        || (*current_index == exchange_index && *current_min != price) =>
                {
                    *quard = Some((exchange_index, price));
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
    fn update_sell_price(&self, exchange_index: usize, price: f64) {
        match self.max_sell_price.write() {
            Ok(mut quard) => match &*quard {
                None => *quard = Some((exchange_index, price)),
                Some((current_index, current_max))
                    if price > *current_max
                        || (*current_index == exchange_index && *current_max != price) =>
                {
                    *quard = Some((exchange_index, price))
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}
