use crate::config::QUOTE_LIST;

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
        // println!("Не удалось распарсить пару {}", symbol);
        None
    }
    pub fn from_str_with_separator(symbol: &str, separator: char) -> Option<Self> {
        symbol.split_once(separator).and_then(|(base, quote)| {
            if base.is_empty() || quote.is_empty() {
                return None;
            }
            for q in QUOTE_LIST {
                if quote.to_uppercase() == *q {
                    return Some(Self::new(base, quote));
                }
            }
            None
        })
    }
}
