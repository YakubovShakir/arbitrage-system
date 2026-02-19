use crate::core::types::ExchangeName;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct ExchangePrice {
    pub exchange: ExchangeName,
    pub price: RwLock<f64>,
    pub checked: RwLock<bool>,
}

impl ExchangePrice {
    pub fn new(exchange: &str, price: &f64) -> Self {
        ExchangePrice {
            exchange: exchange.to_owned(),
            price: RwLock::new(price.to_owned()),
            checked: RwLock::new(false),
        }
    }

    pub async fn get_price(&self) -> f64 {
        *self.price.read().await
    }
    pub async fn check(&self) -> bool {
        let mut checked_guard = self.checked.write().await;
        if *checked_guard {
            false
        } else {
            *checked_guard = true;
            true
        }
    }
}
#[derive(Debug)]
pub struct PriceData {
    pub sell_price_list: Vec<ExchangePrice>,
    pub buy_price_list: Vec<ExchangePrice>,
}

impl PriceData {
    pub fn new(buy_ex_name: &str, sell_ex_name: &str, sell_price: &f64, buy_price: &f64) -> Self {
        Self {
            sell_price_list: vec![ExchangePrice::new(sell_ex_name, sell_price)],
            buy_price_list: vec![ExchangePrice::new(buy_ex_name, buy_price)],
        }
    }
    pub async fn update_buy_list(&mut self, exchange: &str, price: f64) {
        let mut finded = false;
        for exchange_price in &self.buy_price_list {
            if exchange_price.exchange == exchange {
                let mut price_quard = exchange_price.price.write().await;
                let mut checked_quard = exchange_price.checked.write().await;
                *price_quard = price;
                *checked_quard = false;
                finded = true;
                break;
            }
        }
        if !finded {
            self.buy_price_list
                .push(ExchangePrice::new(exchange, &price));
        }
    }

    pub async fn update_sell_list(&mut self, exchange: &str, price: f64) {
        let mut finded = false;
        for exchange_price in &self.sell_price_list {
            if exchange_price.exchange == exchange {
                let mut price_quard = exchange_price.price.write().await;
                let mut checked_quard = exchange_price.checked.write().await;
                *price_quard = price;
                *checked_quard = false;
                finded = true;
                break;
            }
        }
        if !finded {
            self.sell_price_list
                .push(ExchangePrice::new(exchange, &price));
        }
    }
}
