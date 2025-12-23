use std::{sync::Arc, time::Instant};

use futures_util::future::join_all;

use crate::core::{
    traits::{Workable, exchange_service::TickerService},
    types::{Exchanges, TradingPairExchangesBlacklist, TradingPairs},
    utils::format_duration,
};

pub struct TickerWorker {
    id: usize,
    trading_pairs: Arc<TradingPairs>,
    exchanges: Arc<Exchanges>,
    tickers_exchanges_blacklist: Arc<TradingPairExchangesBlacklist>,
}

impl TickerWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<TradingPairs>,
        exchanges: Arc<Exchanges>,
        tickers_exchanges_blacklist: Arc<TradingPairExchangesBlacklist>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            exchanges,
            tickers_exchanges_blacklist,
        }
    }
    async fn fetch_tickers(&self) -> Vec<(String, TradingPairs)> {
        let tickers_futures: Vec<_> = self
            .exchanges
            .iter()
            .map(|(exchange_name, exchange)| async move {
                match exchange.tickers().await {
                    Ok(tickers) => Some((exchange_name.clone(), tickers)),
                    Err(e) => {
                        println!(
                            "Не удалось получить тикеры от биржи {}, ошибка - {}",
                            exchange_name, e
                        );
                        None
                    }
                }
            })
            .collect();
        let tickers_results: Vec<_> = join_all(tickers_futures)
            .await
            .into_iter()
            .filter_map(|result| result)
            .collect();

        tickers_results
    }

    async fn update_tickers(&self, fetched_tickers: Vec<(String, TradingPairs)>) {
        for (exchange_name, tickers) in fetched_tickers {
            for (new_trading_pair, new_price_data) in tickers {
                let mut in_buy_blacklist = false;
                let mut in_sell_blacklist = false;

                if let Some(blacklist) = self.tickers_exchanges_blacklist.get(&new_trading_pair) {
                    if blacklist
                        .buy_exchanges
                        .read()
                        .await
                        .get(&exchange_name)
                        .is_some()
                    {
                        in_buy_blacklist = true
                    };

                    if blacklist
                        .sell_exchanges
                        .read()
                        .await
                        .get(&exchange_name)
                        .is_some()
                    {
                        in_sell_blacklist = true
                    };
                };
                if self.trading_pairs.contains_key(&new_trading_pair) {
                    if let Some(existing_price) = self.trading_pairs.get(&new_trading_pair) {
                        let (buy_price, sell_price) = {
                            let buy_guard = new_price_data.min_buy_price.read().await;
                            let sell_guard = new_price_data.max_sell_price.read().await;
                            (buy_guard.1, sell_guard.1)
                        };

                        if !in_buy_blacklist {
                            existing_price
                                .update_buy_price(exchange_name.clone(), buy_price)
                                .await;
                        }
                        if !in_sell_blacklist {
                            existing_price
                                .update_sell_price(exchange_name.clone(), sell_price)
                                .await;
                        }
                    }
                } else {
                    if !in_buy_blacklist && !in_sell_blacklist {
                        self.trading_pairs.insert(new_trading_pair, new_price_data);
                    }
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
        let mut loop_count = 0;
        let loop_to_update_stat = 10;
        let mut total_elapsed = 0;
        let mut stat_info = format!("----TickerWorker:{} Statistics ----", self.id);
        loop {
            stat_info += &format!("\n       loop: {}", loop_count + 1);

            let start = Instant::now();
            stat_info += &format!("\n| {:<8} | {:<8} |", "Exchange", "Tickers");

            // Получение тикеров с бирж
            let fetched_tickers: Vec<(String, TradingPairs)> = self.fetch_tickers().await;
            for (ex, tickers) in &fetched_tickers {
                stat_info += &format!("\n| {:<8} | {:<8} |", ex, tickers.len());
            }
            // Обновление глобальной мапы
            self.update_tickers(fetched_tickers).await;
            loop_count += 1;
            total_elapsed += start.elapsed().as_nanos();
            if loop_count == loop_to_update_stat {
                stat_info += &format!(
                    "\n---- Total elapsed:{} ----",
                    format_duration(total_elapsed)
                );

                println!("{}", stat_info);
                total_elapsed = 0;
                loop_count = 0;
                stat_info = format!("----TickerWorker:{} statistics ----", self.id);
            }

            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        }
    }
}
