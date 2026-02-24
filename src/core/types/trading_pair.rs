use std::hash::{Hash, Hasher};

use dashmap::DashSet;

use crate::{config::QUOTE_LIST, core::types::ExchangeName};

#[derive(Debug, Clone)]
pub struct TradingPairBlacklist {
    pub buy_exchanges: DashSet<ExchangeName>,
    pub sell_exchanges: DashSet<ExchangeName>,
}

impl TradingPairBlacklist {
    pub fn new() -> Self {
        Self {
            buy_exchanges: DashSet::<String>::new(),
            sell_exchanges: DashSet::<String>::new(),
        }
    }
    pub fn is_buy_blacklisted(&self, exchange_name: &str) -> bool {
        self.buy_exchanges.get(exchange_name).is_some()
    }
    pub fn is_sell_blacklisted(&self, exchange_name: &str) -> bool {
        self.sell_exchanges.get(exchange_name).is_some()
    }
    pub fn blacklist_buy(&self, exchange_name: &str) {
        self.buy_exchanges.insert(exchange_name.to_owned());
    }
    pub fn blacklist_sell(&self, exchange_name: &str) {
        self.sell_exchanges.insert(exchange_name.to_owned());
    }
}

#[derive(Debug, Clone)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
    pub blacklist: TradingPairBlacklist,
}

impl Hash for TradingPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.quote.hash(state);
    }
}

impl PartialEq for TradingPair {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.quote == other.quote
    }
}
impl Eq for TradingPair {}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: base.to_string().to_uppercase(),
            quote: quote.to_string().to_uppercase(),
            blacklist: TradingPairBlacklist::new(),
        }
    }
    pub fn get_volume_by_quote(&self) -> Option<f64> {
        for &(symbol, volume) in QUOTE_LIST {
            if symbol == self.quote {
                return Some(volume);
            }
        }
        None
    }
    pub fn from_str(symbol: &str) -> Option<Self> {
        let upper_symbol = symbol.to_uppercase();
        for quote in QUOTE_LIST {
            let quote = quote.0;
            if upper_symbol.ends_with(quote) {
                let base = &upper_symbol[..symbol.len() - quote.len()];
                if !base.is_empty() {
                    return Some(Self::new(base, quote));
                }
            }
        }
        // error!("Не удалось распарсить пару {}", symbol);
        None
    }
    pub fn from_str_with_separator(symbol: &str, separator: char) -> Option<Self> {
        symbol.split_once(separator).and_then(|(base, quote)| {
            if base.is_empty() || quote.is_empty() {
                return None;
            }
            for q in QUOTE_LIST {
                let q = q.0;

                if quote.to_uppercase() == *q {
                    return Some(Self::new(base, quote));
                }
            }
            None
        })
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new("BTC", "USDT")
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::TradingPair;

    #[test]
    fn test_from_str() {
        let valid = TradingPair::from_str("btcusdt");
        let invalid = TradingPair::from_str("btcusdt-ff");

        assert!(valid.is_some());
        assert!(invalid.is_none());
    }

    #[test]
    fn test_from_str_with_separator() {
        let valid = TradingPair::from_str_with_separator("btc-usdt", '-');
        let invalid = TradingPair::from_str_with_separator("btcusdt", '-');

        assert!(valid.is_some());
        assert!(invalid.is_none());
    }

    #[test]
    fn test_get_volume_by_quote() {
        let valid = TradingPair::mock();
        let invalid = TradingPair::new("BTC", "ZEBRA");

        assert!(valid.get_volume_by_quote().is_some());
        assert!(invalid.get_volume_by_quote().is_none());
    }
}
