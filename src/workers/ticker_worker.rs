use std::{sync::Arc, time::Instant};

use crate::core::{
    traits::{Workable, exchange_service::TickerService},
    types::{Exchanges, TradingPairs},
};

pub struct TickerWorker {
    id: usize,
    trading_pairs: Arc<TradingPairs>,
    exchanges: Arc<Exchanges>,
}

impl TickerWorker {
    pub fn new(id: usize, trading_pairs: Arc<TradingPairs>, exchanges: Arc<Exchanges>) -> Self {
        Self {
            id,
            trading_pairs,
            exchanges,
        }
    }
    async fn fetch_tickers(&self) -> Vec<(String, TradingPairs)> {
        let mut fetched_tickers: Vec<(String, TradingPairs)> = Vec::new();
        for (exchange_name, exchange) in &*self.exchanges {
            let new_tickers = match exchange.tickers().await {
                Ok(tickers) => tickers,
                Err(e) => {
                    println!(
                        "❌ Не удалось получить тикеры от биржи {}, ошибка - {}",
                        exchange_name, e
                    );
                    continue;
                }
            };
            fetched_tickers.push((exchange_name.to_string(), new_tickers));
        }
        fetched_tickers
    }

    async fn update_tickers(&self, fetched_tickers: Vec<(String, TradingPairs)>) {
        for (exchange_name, tickers) in fetched_tickers {
            for (new_trading_pair, new_price_data) in tickers {
                if self.trading_pairs.contains_key(&new_trading_pair) {
                    if let Some(existing_price) = self.trading_pairs.get(&new_trading_pair) {
                        let (buy_price, sell_price) = {
                            let buy_guard = new_price_data.min_buy_price.read().await;
                            let sell_guard = new_price_data.max_sell_price.read().await;
                            (buy_guard.1, sell_guard.1)
                        };

                        existing_price
                            .update_buy_price(exchange_name.clone(), buy_price)
                            .await;
                        existing_price
                            .update_sell_price(exchange_name.clone(), sell_price)
                            .await;
                    }
                } else {
                    self.trading_pairs.insert(new_trading_pair, new_price_data);
                }
            }
        }
    }
}

impl Workable for TickerWorker {
    fn id(&self) -> usize {
        self.id
    }

    async fn run(&self) -> ! {
        loop {
            let start = Instant::now();

            let mut stat_info = format!(
                "---------- Statistic from TickerWorker:{} ----------",
                self.id
            );
            stat_info += &format!("\n| {:<8} | {:<8} |", "Exchange", "Tickers");

            // Получение тикеров с бирж
            let fetched_tickers: Vec<(String, TradingPairs)> = self.fetch_tickers().await;
            for (ex, tickers) in &fetched_tickers {
                stat_info += &format!("\n| {:<8} | {:<8} |", ex, tickers.len());
            }
            // Обновление глобальной мапы
            self.update_tickers(fetched_tickers).await;

            let elapsed = start.elapsed();
            stat_info += &format!("\n---------- Total elapsed:{:?} ----------", elapsed);
            println!("{}", stat_info);
            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        }
    }
}
