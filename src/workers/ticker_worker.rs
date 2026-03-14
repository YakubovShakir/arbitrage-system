use core::f64;
use std::{collections::HashMap, sync::Arc, time::Instant};

use futures_util::future::join_all;

use crate::{
    config::parameters::BASE_BLACKLIST,
    core::{
        traits::{
            Workable,
            exchange_service::{FuturesTickerService, TickerService},
        },
        types::{
            ExchangeName, Exchanges, PriceData, Tickers, TradingPair, TradingPairs,
            price_data::ExchangePrice,
        },
        utils::format_duration,
    },
};
use log::{debug, error, info};
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
    // Получение набора тикеров с каждой биржи вектор хеш-мап, где каждая хеш мапа это тикеры с одной биржи
    async fn fetch_spot_tickers(&self) -> Vec<(ExchangeName, Tickers)> {
        let tickers_futures: Vec<_> = self
            .exchanges
            .iter()
            .map(|(ex_name, exchange)| async move {
                match exchange.tickers().await {
                    Ok(tickers) => Some((ex_name.clone(), tickers)),
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

    async fn fetch_futures_tickers(&self) -> Vec<(ExchangeName, Tickers)> {
        let tickers_futures: Vec<_> = self
            .exchanges
            .iter()
            .map(|(ex_name, exchange)| async move {
                match exchange.futures_tickers().await {
                    Ok(tickers) => Some((ex_name.clone(), tickers)),
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

    // Собираем набор уникальных торговых пар с каждого набора со всех бирж в единую хеш мапу
    // Делается здесь, а не в update_trading_pairs чтобы не брать большое количество блокировок на self.trading_pairs

    async fn collect_tickers(
        &self,
        ex_spot_tickers: Vec<(ExchangeName, Tickers)>,
        ex_futures_tickers: Vec<(ExchangeName, Tickers)>,
    ) -> HashMap<TradingPair, PriceData> {
        let mut collection: HashMap<TradingPair, PriceData> = HashMap::new();

        // Проходимся по вектору биржа->тикеры
        for (ex_name, tickers) in ex_spot_tickers {
            // Проходимся по тикерам каджой биржи
            for (pair, ticker_price) in tickers {
                // Ни с одной биржи не нужен токен из блеклиста
                if BASE_BLACKLIST.contains(&pair.base.as_str()) {
                    continue;
                }
                if let Some(collection_price_data) = collection.get_mut(&pair) {
                    collection_price_data
                        .update_buy_list(&ex_name, ticker_price.buy_price.1)
                        .await;
                    // collection_price_data
                    //     .update_sell_list(&ex_name, ticker_price.sell_price.1)
                    //     .await;
                } else {
                    collection.insert(
                        pair,
                        PriceData::new(&ex_name, &ex_name, &f64::MIN, &ticker_price.buy_price.1),
                    );
                }
            }
        }

        // Проходимся по вектору биржа->тикеры
        for (ex_name, tickers) in ex_futures_tickers {
            // Проходимся по тикерам каджой биржи
            for (pair, ticker_price) in tickers {
                // Ни с одной биржи не нужен токен из блеклиста
                if BASE_BLACKLIST.contains(&pair.base.as_str()) {
                    continue;
                }
                if let Some(collection_price_data) = collection.get_mut(&pair) {
                    // collection_price_data
                    //     .update_buy_list(&ex_name, ticker_price.buy_price.1)
                    //     .await;
                    collection_price_data
                        .update_sell_list(&ex_name, ticker_price.sell_price.1)
                        .await;
                } else {
                    collection.insert(
                        pair,
                        PriceData::new(&ex_name, &ex_name, &ticker_price.sell_price.1, &f64::MAX),
                    );
                }
            }
        }
        collection
    }

    async fn update_trading_pairs(&self, tickers: HashMap<TradingPair, PriceData>) {
        // Взятие торговую пару из коллекции
        for (ticker_pair, ticker_price) in tickers {
            // Проверка на существование в глобальной мапе торговых пар
            if let Some(mut entry) = self.trading_pairs.get_mut(&ticker_pair) {
                // Прохождение по тикерам покупки

                for ExchangePrice {
                    exchange, price, ..
                } in ticker_price.buy_price_list
                {
                    let in_blacklist = entry.key().blacklist.is_buy_blacklisted(&exchange);
                    if in_blacklist {
                        // debug!(target: "debug_module", "From global map trading pairs: {:#?} \n {:#?} ",entry.key(), entry.value());

                        // debug!(target: "debug_module", "Exchange {} in buy blacklist of {}/{}", exchange, ticker_pair.base, ticker_pair.quote);
                        continue;
                    }
                    entry
                        .value_mut()
                        .update_buy_list(&exchange, *price.read().await)
                        .await;
                }

                // Прохождение по тикерам продажи
                for ExchangePrice {
                    exchange, price, ..
                } in ticker_price.sell_price_list
                {
                    let in_blacklist = entry.key().blacklist.is_sell_blacklisted(&exchange);
                    if in_blacklist {
                        // debug!(target: "debug_module", "Exchange {} in sell blacklist of {}/{}, {:#?}", exchange, ticker_pair.base, ticker_pair.quote, entry.key().blacklist);
                        // debug!(target: "debug_module", "From global map trading pairs: {:#?} \n {:#?} ",entry.key(), entry.value());

                        continue;
                    }
                    entry
                        .value_mut()
                        .update_sell_list(&exchange, *price.read().await)
                        .await;
                }
            } else {
                self.trading_pairs.insert(ticker_pair, ticker_price);
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
            let raw_spot_tickers: Vec<(ExchangeName, Tickers)> = self.fetch_spot_tickers().await;
            let raw_futures_tickers: Vec<(ExchangeName, Tickers)> =
                self.fetch_futures_tickers().await;

            // debug!(target: "debug_module", "Raw Tickers: {:#?} ",raw_tickers);
            for (ex_name, tickers) in &raw_spot_tickers {
                if let Some(count) = stat_map.get_mut(ex_name) {
                    *count += tickers.len();
                } else {
                    stat_map.insert(ex_name.to_owned(), tickers.len());
                };
            }

            let collected_tickers: HashMap<TradingPair, PriceData> = self
                .collect_tickers(raw_spot_tickers, raw_futures_tickers)
                .await;
            // debug!(target: "debug_module", "Collected Tickers: {:#?} ",collected_tickers);

            self.update_trading_pairs(collected_tickers).await;

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
