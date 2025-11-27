use crate::{config::QUOTE_LIST, core::types::ExchangeName};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Network {
    pub name: String,
    pub full_name: String,
    pub coin_name: String,
    pub withdraw_fee: Option<f64>,
    pub contract_address: Option<String>,
    pub memo: Option<String>,
    pub deposit_address: Option<String>,
}

impl Network {
    pub fn new(
        name: String,
        full_name: String,
        coin_name: String,
        withdraw_fee: Option<f64>,
        contract_address: Option<String>,
        memo: Option<String>,
        deposit_address: Option<String>,
    ) -> Self {
        let contract_address = contract_address.filter(|addr| !addr.is_empty());
        let memo = memo.filter(|m| !m.is_empty());
        let deposit_address = deposit_address.filter(|addr| !addr.is_empty());

        Network {
            name,
            full_name,
            coin_name,
            withdraw_fee,
            contract_address,
            memo: memo,
            deposit_address,
        }
    }
    pub fn create_test() -> Network {
        Network::new(
            "Test Network name".to_string(),
            "Test Network full_name".to_string(),
            "Test Network coin".to_string(),
            None,
            None,
            None,
            None,
        )
    }
}
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
            if !base.is_empty() && !quote.is_empty() {
                Some(Self::new(base, quote))
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub struct PriceData {
    pub min_buy_price: RwLock<(ExchangeName, f64)>,
    pub max_sell_price: RwLock<(ExchangeName, f64)>,
    checked: RwLock<bool>,
}

impl PriceData {
    pub fn new(sell_price: (ExchangeName, f64), buy_price: (ExchangeName, f64)) -> Self {
        Self {
            max_sell_price: RwLock::new(sell_price),
            min_buy_price: RwLock::new(buy_price),
            checked: RwLock::new(false),
        }
    }
}

impl PriceData {
    pub async fn update_buy_price(&self, exchange_name: String, price: f64) {
        let mut data_quard = self.min_buy_price.write().await;

        let price_is_better = price < data_quard.1;
        let current_exchange_price_changed = data_quard.0 == exchange_name && data_quard.1 != price;

        if price_is_better || current_exchange_price_changed {
            *data_quard = (exchange_name, price);

            let mut checked_quard = self.checked.write().await;
            *checked_quard = false;
        }
    }
    pub async fn update_sell_price(&self, exchange_name: String, price: f64) {
        let mut data_guard = self.max_sell_price.write().await;

        let price_is_better = price > data_guard.1;
        let current_exchange_price_changed = data_guard.0 == exchange_name && data_guard.1 != price;

        if price_is_better || current_exchange_price_changed {
            *data_guard = (exchange_name, price);

            let mut checked_quard = self.checked.write().await;
            *checked_quard = false;
        }
    }

    pub async fn check_if_not_checked(&self) -> bool {
        let mut checked_guard = self.checked.write().await;
        if *checked_guard {
            *checked_guard
        } else {
            *checked_guard = true;
            false
        }
    }
}
