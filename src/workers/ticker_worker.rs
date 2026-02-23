use std::{collections::HashMap, sync::Arc, time::Instant};

use futures_util::future::join_all;

use crate::{
    config::parameters::BASE_BLACKLIST,
    core::{
        traits::{Workable, exchange_service::TickerService},
        types::{Exchanges, PriceData, TickerPrice, Tickers, TradingPairs, blacklist::Blacklist},
        utils::format_duration,
    },
};
use log::{error, info};
pub struct TickerWorker {
    id: usize,
    trading_pairs: Arc<TradingPairs>,
    exchanges: Arc<Exchanges>,
    blacklist: Arc<Blacklist>,
}

impl TickerWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<TradingPairs>,
        exchanges: Arc<Exchanges>,
        blacklist: Arc<Blacklist>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            exchanges,
            blacklist,
        }
    }
    async fn fetch_tickers(&self) -> Vec<Tickers> {
        let tickers_futures: Vec<_> = self
            .exchanges
            .iter()
            .map(|(ex_name, exchange)| async move {
                match exchange.tickers().await {
                    Ok(tickers) => Some(tickers),
                    Err(e) => {
                        error!(
                            "Не удалось получить тикеры от биржи {}, ошибка - {}",
                            ex_name, e
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

    async fn filter_tickers(&self, raw_tickers: Vec<Tickers>) -> Tickers {
        let mut filtered: Tickers = Tickers::new();

        raw_tickers.iter().for_each(|tickers| {
            for (pair, price) in tickers {
                if BASE_BLACKLIST.contains(&pair.base.as_str()) {
                    continue;
                }
                let ex_name = &price.buy_price.0;
                let in_buy_blacklist = self.blacklist.is_buy_blacklisted(pair, &ex_name);
                let in_sell_blacklist = self.blacklist.is_sell_blacklisted(pair, &ex_name);

                if let Some(filtered_ticker) = filtered.get_mut(pair) {
                    if !in_buy_blacklist {
                        if price.buy_price.1 < filtered_ticker.buy_price.1 {
                            filtered_ticker.buy_price = price.buy_price.clone()
                        }
                    }
                    if !in_sell_blacklist {
                        if price.sell_price.1 > filtered_ticker.sell_price.1 {
                            filtered_ticker.sell_price = price.sell_price.clone()
                        }
                    }
                } else {
                    // Создаем новый тикер, даже если только одна сторона валидна
                    let mut new_ticker = TickerPrice {
                        buy_price: (String::new(), f64::MAX),
                        sell_price: (String::new(), f64::MIN),
                    };

                    if !in_buy_blacklist {
                        new_ticker.buy_price = price.buy_price.clone();
                    }
                    if !in_sell_blacklist {
                        new_ticker.sell_price = price.sell_price.clone();
                    }

                    // Добавляем только если хотя бы одна сторона валидна
                    if !new_ticker.buy_price.0.is_empty() || !new_ticker.sell_price.0.is_empty() {
                        filtered.insert(pair.clone(), new_ticker);
                    }
                }
            }
        });
        filtered
    }
    async fn update_trading_pairs(&self, tickers: Tickers) {
        for (pair, price) in tickers {
            let buy_ex_name = price.buy_price.0;
            let sell_ex_name = price.sell_price.0;
            let price_data = PriceData::new(
                &buy_ex_name,
                &sell_ex_name,
                &price.sell_price.1,
                &price.buy_price.1,
            );

            if let Some(mut existing_price) = self.trading_pairs.get_mut(&pair) {
                existing_price
                    .update_buy_list(&buy_ex_name, price.buy_price.1)
                    .await;

                existing_price
                    .update_sell_list(&sell_ex_name, price.sell_price.1)
                    .await;
            } else {
                self.trading_pairs.insert(pair, price_data);
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
        let mut stat_info = format!(
            "----TickerWorker:{} Statistics ----\n| {:<8} | {:<8} |",
            self.id, "Exchange", "Tickers"
        );
        let mut stat_map: HashMap<String, usize> = HashMap::new();

        loop {
            let start = Instant::now();

            // Получение тикеров с бирж
            let raw_tickers: Vec<Tickers> = self.fetch_tickers().await;
            for tickers in &raw_tickers {
                if let Some(pair) = tickers.iter().nth(0) {
                    if let Some(count) = stat_map.get_mut(&pair.1.buy_price.0) {
                        *count += tickers.len();
                    } else {
                        stat_map.insert(pair.1.buy_price.0.clone(), tickers.len());
                    };
                }
            }

            let filtered_tickers: Tickers = self.filter_tickers(raw_tickers).await;
            self.update_trading_pairs(filtered_tickers).await;

            loop_count += 1;
            total_elapsed += start.elapsed().as_nanos();
            if loop_count == loop_to_update_stat {
                for (exchange_name, count) in &stat_map {
                    stat_info += &format!("\n| {:<8} | {:<8} |", exchange_name, count);
                }
                stat_info += &format!(
                    "\n---- Total elapsed:{} ----",
                    format_duration(total_elapsed)
                );

                info!(target:"info_module", "{}", stat_info);
                total_elapsed = 0;
                loop_count = 0;
                stat_map.clear();
                stat_info = format!(
                    "----TickerWorker:{} Statistics ----\n| {:<8} | {:<8} |",
                    self.id, "Exchange", "Tickers"
                );
            }

            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }
    }
}
