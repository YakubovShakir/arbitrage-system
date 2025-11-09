use std::sync::RwLock;

use crate::{config::QUOTE_LIST, core::types::ExchangeName};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: base.to_string().to_uppercase(),
            quote: quote.to_string().to_uppercase(),
        }
    }
    pub fn from_str(symbol: &str) -> Option<Self> {
        let upper_symbol = symbol.to_uppercase();
        for quote in QUOTE_LIST {
            if upper_symbol.ends_with(quote) {
                let base = &upper_symbol[..symbol.len() - quote.len()];
                if !base.is_empty() {
                    return Some(Self::new(base, quote));
                }
            }
        }
        println!("Не удалось распарсить пару {}", symbol);
        None
    }
}

#[derive(Debug)]
pub struct PriceData {
    min_buy_price: RwLock<Option<(ExchangeName, f64)>>,
    max_sell_price: RwLock<Option<(ExchangeName, f64)>>,
}

impl PriceData {
    pub fn new(sell_price: (ExchangeName, f64), buy_price: (ExchangeName, f64)) -> Self {
        Self {
            max_sell_price: RwLock::new(Some(sell_price)),
            min_buy_price: RwLock::new(Some(buy_price)),
        }
    }
}

impl PriceData {
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
            Ok(mut quard) => match &*quard {
                None => *quard = Some((exchange_name, price)),
                Some((current_name, current_min))
                    if price < *current_min
                        || (*current_name == exchange_name && *current_min != price) =>
                {
                    *quard = Some((exchange_name, price));
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
    pub fn update_sell_price(&self, exchange_name: String, price: f64) {
        match self.max_sell_price.write() {
            Ok(mut quard) => match &*quard {
                None => *quard = Some((exchange_name, price)),
                Some((current_name, current_max))
                    if price > *current_max
                        || (*current_name == exchange_name && *current_max != price) =>
                {
                    *quard = Some((exchange_name, price))
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}
