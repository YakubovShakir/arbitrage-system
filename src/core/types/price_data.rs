use crate::core::types::ExchangeName;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct PriceData {
    pub min_buy_price: RwLock<(ExchangeName, f64)>,
    pub max_sell_price: RwLock<(ExchangeName, f64)>,
    checked: RwLock<bool>,
}

impl PriceData {
    pub fn new(exchange_name: &str, sell_price: &f64, buy_price: &f64) -> Self {
        Self {
            min_buy_price: RwLock::new((exchange_name.to_owned(), buy_price.to_owned())),
            max_sell_price: RwLock::new((exchange_name.to_owned(), sell_price.to_owned())),
            checked: RwLock::new(false),
        }
    }
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
