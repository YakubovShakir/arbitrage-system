use crate::{
    config::parameters::REQUIRED_TICKER_SPREAD_PERCENT,
    core::{
        types::{ExchangeName, trading_pair::TradingPairBlacklist},
        utils::comput_spread_percent,
    },
};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct TickerSpread {
    pub buy_ex: ExchangeName,
    pub sell_ex: ExchangeName,
}

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
    pub async fn detect_spread_pairs(
        &self,
        blacklist: &TradingPairBlacklist,
    ) -> Option<Vec<TickerSpread>> {
        let mut spread_pairs: Vec<TickerSpread> = Vec::new();

        // Фильтруем buy_price_list и sell_price_list, исключая заблокированные биржи
        let active_buy: Vec<_> = self
            .buy_price_list
            .iter()
            .filter(|p| !blacklist.is_buy_blacklisted(&p.exchange))
            .collect();

        let active_sell: Vec<_> = self
            .sell_price_list
            .iter()
            .filter(|p| !blacklist.is_sell_blacklisted(&p.exchange))
            .collect();

        let mut updated_buy = Vec::new();
        let mut not_updated_buy = Vec::new();
        for price in active_buy {
            if price.check().await {
                updated_buy.push(price);
            } else {
                not_updated_buy.push(price);
            }
        }

        let mut updated_sell = Vec::new();
        for price in &active_sell {
            if price.check().await {
                updated_sell.push(price);
            }
        }
        for buy_price in &updated_buy {
            for sell_price in &active_sell {
                if buy_price.exchange == sell_price.exchange {
                    continue;
                }
                let spread = comput_spread_percent(
                    &buy_price.get_price().await,
                    &sell_price.get_price().await,
                );
                if spread >= REQUIRED_TICKER_SPREAD_PERCENT {
                    spread_pairs.push(TickerSpread {
                        buy_ex: buy_price.exchange.clone(),
                        sell_ex: sell_price.exchange.clone(),
                    });
                }
            }
        }

        // Второй проход: updated_sell × not_updated_buy (только активные buy)
        for sell_price in &updated_sell {
            for buy_price in &not_updated_buy {
                if buy_price.exchange == sell_price.exchange {
                    continue;
                }
                let spread = comput_spread_percent(
                    &buy_price.get_price().await,
                    &sell_price.get_price().await,
                );
                if spread >= REQUIRED_TICKER_SPREAD_PERCENT {
                    spread_pairs.push(TickerSpread {
                        buy_ex: buy_price.exchange.clone(),
                        sell_ex: sell_price.exchange.clone(),
                    });
                }
            }
        }

        if spread_pairs.len() > 0 {
            Some(spread_pairs)
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        let buy_list = vec![
            ExchangePrice::new("Binance", &100.0),
            ExchangePrice::new("Bybit", &105.0),
            ExchangePrice::new("Gate.io", &100.5),
            ExchangePrice::new("Kucoin", &99.0),
        ];
        let sell_list = vec![
            ExchangePrice::new("Binance", &99.0),
            ExchangePrice::new("Bybit", &100.5),
            ExchangePrice::new("Gate.io", &105.5),
            ExchangePrice::new("Huobi", &102.0),
        ];
        PriceData {
            buy_price_list: buy_list,
            sell_price_list: sell_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::{
        PriceData, price_data::ExchangePrice, trading_pair::TradingPairBlacklist,
    };

    #[tokio::test]
    async fn test_detect_spread_pairs() {
        let mut mock = PriceData::mock();
        let spread = mock
            .detect_spread_pairs(&TradingPairBlacklist::new())
            .await
            .unwrap();
        assert!(spread.len() == 8);
        assert!(
            mock.detect_spread_pairs(&TradingPairBlacklist::new())
                .await
                .is_none()
        );

        let _ = mock.update_sell_list("Binance", 101.0).await;
        let spread = mock
            .detect_spread_pairs(&TradingPairBlacklist::new())
            .await
            .unwrap();
        assert!(spread.len() == 2);

        let _ = mock.update_buy_list("Binance", 101.0).await;
        let spread = mock
            .detect_spread_pairs(&TradingPairBlacklist::new())
            .await
            .unwrap();
        assert!(spread.len() == 2);

        let _ = mock.update_buy_list("Bybit", 103.0).await;
        let _ = mock.update_sell_list("Bybit", 103.0).await;
        let spread = mock
            .detect_spread_pairs(&TradingPairBlacklist::new())
            .await
            .unwrap();
        assert!(spread.len() == 4);
    }

    #[tokio::test]
    async fn test_update_buy_list() {
        let mut mock = PriceData::mock();
        let buy_list_len = mock.buy_price_list.len();

        // add existing exchange price
        mock.update_buy_list("Binance", 101.0).await;
        assert!(mock.buy_price_list.len() == buy_list_len);

        for ExchangePrice {
            exchange,
            price,
            checked,
        } in &mock.buy_price_list
        {
            if exchange == "Binance" {
                assert!(*price.read().await == 101.0);
                assert!(*checked.read().await == false);
            }
        }

        // add new exchange price
        mock.update_buy_list("OKX", 103.1).await;
        assert!(mock.buy_price_list.len() == buy_list_len + 1);
    }

    #[tokio::test]
    async fn test_update_sell_list() {
        let mut mock = PriceData::mock();
        let sell_list_len = mock.sell_price_list.len();

        // add existing exchange price
        mock.update_sell_list("Binance", 101.0).await;
        assert!(mock.sell_price_list.len() == sell_list_len);

        for ExchangePrice {
            exchange,
            price,
            checked,
        } in &mock.sell_price_list
        {
            if exchange == "Binance" {
                assert!(*price.read().await == 101.0);
                assert!(*checked.read().await == false);
            }
        }
        // add new exchange price
        mock.update_sell_list("OKX", 103.1).await;
        assert!(mock.sell_price_list.len() == sell_list_len + 1);
    }
}
