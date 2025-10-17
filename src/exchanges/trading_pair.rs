use std::sync::RwLock;
pub struct TradingPair {
    base: String,
    quote: String,

    min_buy_price: RwLock<Option<(String, f64)>>,
    max_sell_price: RwLock<Option<(String, f64)>>,
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

    pub fn base(&self) -> &str {
        &self.base
    }
    pub fn quote(&self) -> &str {
        &self.quote
    }
    pub fn min_buy_price(&self) -> Option<(String, f64)> {
        match self.min_buy_price.read() {
            Ok(quard) => quard.clone(),
            Err(_) => None,
        }
    }
    pub fn max_sell_price(&self) -> Option<(String, f64)> {
        match self.max_sell_price.read() {
            Ok(quard) => quard.clone(),
            Err(_) => None,
        }
    }

    pub fn update_buy_price(&self, exchange_name: String, price: f64) {
        match self.min_buy_price.write() {
            Ok(mut quard) => match *quard {
                None => *quard = Some((exchange_name, price)),
                Some((_, current_min)) if price < current_min => {
                    *quard = Some((exchange_name, price));
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
    pub fn update_sell_price(&self, exchange_name: String, price: f64) {
        match self.max_sell_price.write() {
            Ok(mut quard) => match *quard {
                None => *quard = Some((exchange_name, price)),
                Some((_, current_max)) if price > current_max => {
                    *quard = Some((exchange_name, price))
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}
