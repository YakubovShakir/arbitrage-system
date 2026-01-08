use dashmap::DashSet;
use futures_util::future::join_all;
use tokio::sync::RwLock;

use crate::{
    config::{
        USDT_LIMIT,
        parameters::{REQUIRED_ORDERBOOK_SPREAD_PERCENT, REQUIRED_TICKER_SPREAD_PERCENT},
    },
    core::{
        traits::{Workable, exchange_service::OrderBookService},
        types::{
            Exchanges, Network, OrderBook, TradingPair, TradingPairBlackList,
            TradingPairExchangesBlacklist, TradingPairs, exchanges::Exchange,
        },
        utils::{
            calculate_price_by_glass, comput_spread_percent, format_duration,
            verify_arbitrage_conditions_and_get_networks,
        },
    },
};
use std::{sync::Arc, time::Instant};

pub struct ComputWorker {
    id: usize,
    trading_pairs: Arc<TradingPairs>,
    spread_pairs: Arc<TradingPairs>,
    exchanges: Arc<Exchanges>,
    tickers_exchanges_blacklist: Arc<TradingPairExchangesBlacklist>,
}

impl ComputWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<TradingPairs>,
        spread_pairs: Arc<TradingPairs>,
        exchanges: Arc<Exchanges>,
        tickers_exchanges_blacklist: Arc<TradingPairExchangesBlacklist>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            spread_pairs,
            exchanges,
            tickers_exchanges_blacklist,
        }
    }
}

impl Workable for ComputWorker {
    fn id(&self) -> usize {
        self.id
    }
    // "┌ │ ├─ └─"
    async fn run(&self) -> ! {
        let mut check_passed = 0;
        let mut check_total_elapsed: u128 = 0;

        let mut prices_quard_total_elapsed: u128 = 0;

        let mut ticker_spread_passed = 0;
        let mut ticker_spread_total_elapsed: u128 = 0;

        let mut verify_passed = 0;
        let mut verify_total_elapsed: u128 = 0;

        let mut orderbook_passed = 0;
        let mut orderbook_total_elapsed: u128 = 0;

        let mut calc_books_passed = 0;
        let mut calc_books_total_elapsed: u128 = 0;

        let mut final_spread_passed = 0;
        let mut final_spread_total_elapsed: u128 = 0;

        let mut loop_count = 0;
        let loop_to_update_stat = 10;
        let mut stat_info = format!(
            "┌── СomputWorker:{} Statistics for {} iteration\n",
            self.id, loop_to_update_stat
        );
        let mut loop_elapsed = 0;

        loop {
            let start_time = Instant::now();
            // stat_info += &format!("├──── Trading Pairs: {}\n", self.trading_pairs.len());

            // Сбор непроверенные тикеры
            let mut not_checked_keys: Vec<TradingPair> = vec![];
            for entry in self.trading_pairs.iter() {
                let pair = entry.key();
                let price_data = entry.value();

                if self.spread_pairs.contains_key(pair) {
                    continue;
                }
                let checked_time = Instant::now();
                let checked = price_data.check_if_not_checked().await;
                check_total_elapsed += checked_time.elapsed().as_nanos();
                if checked {
                    continue;
                }
                not_checked_keys.push(pair.clone());
            }
            check_passed += not_checked_keys.len();

            // Фильтрация по тикер спреду
            let mut ticker_spread_passed_keys: Vec<(TradingPair, String, String)> = vec![];
            for pair in not_checked_keys {
                let Some(entry) = self.trading_pairs.get(&pair) else {
                    continue;
                };
                let price_data = entry.value();

                let prices_time = Instant::now();
                let (buy_price, sell_price) = {
                    let buy_guard = price_data.min_buy_price.read().await;
                    let sell_guard = price_data.max_sell_price.read().await;
                    (buy_guard, sell_guard)
                };
                prices_quard_total_elapsed += prices_time.elapsed().as_nanos();

                // Вычисление тикер-спреда
                let ticker_spread_time = Instant::now();
                let spread = comput_spread_percent(&buy_price.1, &sell_price.1);
                ticker_spread_total_elapsed += ticker_spread_time.elapsed().as_nanos();
                if spread < REQUIRED_TICKER_SPREAD_PERCENT {
                    continue;
                }

                ticker_spread_passed_keys.push((
                    pair,
                    buy_price.0.to_string(),
                    sell_price.0.to_string(),
                ));
            }
            ticker_spread_passed += ticker_spread_passed_keys.len();

            // Сбор параметров для параллельного запроса на верификацию
            let verify_tasks: Vec<_> = ticker_spread_passed_keys
                .iter()
                .filter_map(|(pair, buy_exchange_name, sell_exchange_name)| {
                    let (buy_exchange, sell_exchange) = {
                        match (
                            self.exchanges.get(buy_exchange_name),
                            self.exchanges.get(sell_exchange_name),
                        ) {
                            (Some(buy), Some(sell)) => (buy, sell),
                            _ => return None,
                        }
                    };

                    // Создаем задачу
                    Some((pair, buy_exchange, sell_exchange))
                })
                .collect();

            // Параллельное выполнение
            let verify_futures: Vec<_> = verify_tasks
                .into_iter()
                .map(|(pair, buy_exchange, sell_exchange)| async move {
                    match verify_arbitrage_conditions_and_get_networks(
                        &buy_exchange,
                        &sell_exchange,
                        &pair,
                    )
                    .await
                    {
                        Ok(networks) => {
                            if networks.len() != 0 {
                                Some((pair, buy_exchange, sell_exchange, networks))
                            } else {
                                None
                            }
                        }
                        Err(failed_exchange) => {
                            self.trading_pairs.remove(&pair);
                            if let Some(blacklist) = self.tickers_exchanges_blacklist.get(pair) {
                                let blacklist = blacklist.value();
                                if failed_exchange == buy_exchange.config().name {
                                    // println!("[DEBUG] Верификация не пройдена из-за биржи покупки {} для {}/{}", failed_exchange, pair.base, pair.quote);
                                    blacklist.buy_exchanges.insert(failed_exchange.clone());
                                }
                                if failed_exchange == sell_exchange.config().name {
                                    // println!("[DEBUG] Верификация не пройдена из-за биржи продажи {} для {}/{}", failed_exchange, pair.base, pair.quote);

                                    blacklist.sell_exchanges.insert(failed_exchange);
                                };

                                None
                            } else {
                                let blacklist = TradingPairBlackList {
                                    buy_exchanges: DashSet::<String>::new(),
                                    sell_exchanges: DashSet::<String>::new(),
                                };
                                if failed_exchange == buy_exchange.config().name {
                                    // println!("[DEBUG] Верификация не пройдена из-за buy {} для {}/{}", failed_exchange, pair.base, pair.quote);

                                    blacklist.buy_exchanges.insert(failed_exchange.clone());
                                }
                                if failed_exchange == sell_exchange.config().name {
                                    // println!("[DEBUG] Верификация не пройдена из-за sell {} для {}/{}", failed_exchange, pair.base, pair.quote);

                                    blacklist.sell_exchanges.insert(failed_exchange);
                                };
                                self.tickers_exchanges_blacklist
                                    .insert(pair.clone(), blacklist);

                                None
                            }
                        }
                    }
                })
                .collect();

            let verify_time = Instant::now();
            let verify_results: Vec<Option<(&TradingPair, &Exchange, &Exchange, Vec<Network>)>> =
                join_all(verify_futures).await;
            verify_total_elapsed += verify_time.elapsed().as_nanos();

            let verify_passed_pairs: Vec<(&TradingPair, &Exchange, &Exchange, Vec<Network>)> =
                verify_results
                    .into_iter()
                    .filter_map(|result| result)
                    .collect();
            verify_passed += verify_passed_pairs.len();

            let orderbook_futures = verify_passed_pairs.into_iter().map(
                |(pair, buy_exchange, sell_exchange, networks)| async move {
                    let (buy_book, sell_book) = tokio::join!(
                        async {
                            match buy_exchange.orderbook(&pair.base, &pair.quote).await {
                                Ok(book) => Ok(book),
                                Err(e) => {
                                    println!(
                                        "Не удалось получить orderbook с {} - {}",
                                        buy_exchange.config().name,
                                        e
                                    );
                                    Err(())
                                }
                            }
                        },
                        async {
                            match sell_exchange.orderbook(&pair.base, &pair.quote).await {
                                Ok(book) => Ok(book),
                                Err(e) => {
                                    println!(
                                        "Не удалось получить orderbook с {} - {}",
                                        sell_exchange.config().name,
                                        e
                                    );
                                    Err(())
                                }
                            }
                        }
                    );
                    (
                        pair,
                        buy_exchange,
                        sell_exchange,
                        networks,
                        buy_book,
                        sell_book,
                    )
                },
            );
            let books_time = Instant::now();
            let orderbook_results: Vec<(
                &TradingPair,
                &Exchange,
                &Exchange,
                Vec<Network>,
                Result<OrderBook, ()>,
                Result<OrderBook, ()>,
            )> = join_all(orderbook_futures).await;
            orderbook_total_elapsed += books_time.elapsed().as_nanos();

            let orderbook_passed_pairs: Vec<(
                &TradingPair,
                &Exchange,
                &Exchange,
                Vec<Network>,
                OrderBook,
                OrderBook,
            )> = orderbook_results
                .into_iter()
                .filter_map(
                    |(
                        pair,
                        buy_exchange,
                        sell_exchange,
                        networks,
                        buy_orderbook_result,
                        sell_orderbook_result,
                    )| {
                        match (buy_orderbook_result, sell_orderbook_result) {
                            (Ok(buy_orderbook), Ok(sell_orderbook)) => Some((
                                pair,
                                buy_exchange,
                                sell_exchange,
                                networks,
                                buy_orderbook,
                                sell_orderbook,
                            )),
                            _ => None,
                        }
                    },
                )
                .collect();
            orderbook_passed += orderbook_passed_pairs.len();

            for (pair, buy_exchange, sell_exchange, networks, buy_orderbook, sell_orderbook) in
                orderbook_passed_pairs
            {
                let calc_books_time = Instant::now();
                let (calculated_asks, calculated_bids) = (
                    calculate_price_by_glass(&USDT_LIMIT, &buy_orderbook.0),
                    calculate_price_by_glass(&USDT_LIMIT, &sell_orderbook.1),
                );

                calc_books_total_elapsed += calc_books_time.elapsed().as_nanos();
                let (Some(calculated_buy_price), Some(calculated_sell_price)) =
                    (calculated_asks, calculated_bids)
                else {
                    continue;
                };
                calc_books_passed += 1;

                let final_spread_time = Instant::now();
                let final_spread =
                    comput_spread_percent(&calculated_buy_price, &calculated_sell_price);
                final_spread_total_elapsed += final_spread_time.elapsed().as_nanos();

                if final_spread < REQUIRED_ORDERBOOK_SPREAD_PERCENT || final_spread > 10.0 {
                    continue;
                }
                final_spread_passed += 1;

                println!(
                    "✅ {}/{}. \nSpread {:.2}% \nBuy: {} Sell: {} \nNetworks: {:#?}",
                    pair.base,
                    pair.quote,
                    final_spread,
                    buy_exchange.config().name,
                    sell_exchange.config().name,
                    networks
                );
            }
            loop_elapsed += start_time.elapsed().as_nanos();

            loop_count += 1;
            if loop_count == loop_to_update_stat {
                let formated_total_check = format_duration(check_total_elapsed);
                let formated_total_prices = format_duration(prices_quard_total_elapsed);
                let formated_total_ticker = format_duration(ticker_spread_total_elapsed);
                let formated_total_verify = format_duration(verify_total_elapsed);
                let formated_total_orderbook = format_duration(orderbook_total_elapsed);
                let formated_total_calc_books = format_duration(calc_books_total_elapsed);
                let formated_total_final = format_duration(final_spread_total_elapsed);
                let formated_total_loop = format_duration(loop_elapsed);

                // "│ ├─ └─"
                stat_info += &format!("├──── Updated tickers\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_check);
                stat_info += &format!("│        └─ Total passed: {}\n", check_passed);
                stat_info += &format!("├──── Prices stat\n");
                stat_info += &format!("│        └─ Total elapsed: {}\n", formated_total_prices);
                stat_info += &format!("├──── Ticker spread stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_ticker);
                stat_info += &format!("│        └─ Total passed: {}\n", ticker_spread_passed);
                stat_info += &format!("├──── Verify stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_verify);
                stat_info += &format!("│        └─ Total passed: {}\n", verify_passed);
                stat_info += &format!("├──── Orderbook stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_orderbook);
                stat_info += &format!("│        └─ Total passed: {}\n", orderbook_passed);
                stat_info += &format!("├──── Calculated Books stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_calc_books);
                stat_info += &format!("│        └─ Total passed: {}\n", calc_books_passed);
                stat_info += &format!("├──── Final spread stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_final);
                stat_info += &format!("│        └─ Total passed: {}\n", final_spread_passed);
                stat_info += &format!("└── Total elapsed: {}\n", formated_total_loop);

                println!("{}", stat_info);

                loop_count = 0;
                check_passed = 0;
                check_total_elapsed = 0;
                prices_quard_total_elapsed = 0;
                ticker_spread_passed = 0;
                ticker_spread_total_elapsed = 0;
                verify_passed = 0;
                verify_total_elapsed = 0;
                orderbook_passed = 0;
                orderbook_total_elapsed = 0;
                calc_books_passed = 0;
                calc_books_total_elapsed = 0;
                final_spread_passed = 0;
                final_spread_total_elapsed = 0;
                loop_elapsed = 0;
                stat_info = format!(
                    "┌── СomputWorker:{} Statistics for {} iteration\n",
                    self.id, loop_to_update_stat
                );
            }
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }
    }
}
