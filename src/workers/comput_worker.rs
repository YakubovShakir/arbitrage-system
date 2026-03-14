use futures_util::future::join_all;

use crate::{
    config::parameters::{QUOTE_RATE, REQUESTS_CHUNK_SIZE, REQUIRED_ORDERBOOK_SPREAD_PERCENT},
    core::{
        traits::{
            Workable,
            exchange_service::{FuturesOrderBookService, OrderBookService},
        },
        types::{
            Exchanges, OrderBook, TradingPair, TradingPairs,
            exchanges::Exchange,
            network::{DepositNetwork, WithdrawNetwork},
            price_data::TickerSpread,
        },
        utils::{
            calculate_price_by_glass, comput_spread_percent, format_duration,
            verify_arbitrage_conditions_and_get_networks,
        },
    },
};
use log::{debug, error, info};
use std::{collections::HashMap, sync::Arc, time::Instant};

struct SpreadBundle<'a> {
    pair: TradingPair,
    buy_ex: &'a Exchange,
    sell_ex: &'a Exchange,
    buy_price: f64,
    sell_price: f64,
    base_profit: f64,
    quote_volume: f64,
}

pub struct ComputWorker {
    id: usize,
    trading_pairs: Arc<TradingPairs>,
    spread_pairs: Arc<TradingPairs>,
    exchanges: Arc<Exchanges>,
}

impl ComputWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<TradingPairs>,
        spread_pairs: Arc<TradingPairs>,
        exchanges: Arc<Exchanges>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            spread_pairs,
            exchanges,
        }
    }
}
impl ComputWorker {
    async fn find_ticker_spreads(&self) -> (Vec<(TradingPair, Vec<TickerSpread>)>, u128, usize) {
        let mut ticker_spreads: Vec<(TradingPair, Vec<TickerSpread>)> = Vec::new();
        let mut total_bundles: usize = 0;
        let mut comput_elapsed: u128 = 0;

        let comput_time = Instant::now();

        for entry in self.trading_pairs.iter() {
            if let Some(spreads) = entry.detect_spread_pairs(&entry.key().blacklist).await {
                total_bundles += spreads.len();
                ticker_spreads.push((entry.key().clone(), spreads));
            }
        }
        comput_elapsed += comput_time.elapsed().as_nanos();

        (ticker_spreads, comput_elapsed, total_bundles)
    }
}
impl Workable for ComputWorker {
    fn id(&self) -> usize {
        self.id
    }

    // "┌ │ ├─ └─"
    async fn run(&self) -> ! {
        let mut ticker_spread_passed = 0;
        let mut ticker_spread_total_elapsed: u128 = 0;

        let mut orderbook_passed = 0;
        let mut orderbook_total_elapsed: u128 = 0;

        let mut calc_books_passed = 0;
        let mut calc_books_total_elapsed: u128 = 0;

        let mut orderbook_spread_passed = 0;
        let mut orderbook_spread_total_elapsed: u128 = 0;

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

            // Сбор тикер спредов по биржевым пересечениям
            let (pairs, elapsed, total_bundles) = self.find_ticker_spreads().await;
            ticker_spread_total_elapsed += elapsed;
            ticker_spread_passed += total_bundles;

            let normalized_pairs = pairs
                .into_iter()
                .map(|(pair, spreads)| {
                    let mut normalized: Vec<(TradingPair, &Exchange, &Exchange)> = Vec::new();
                    for TickerSpread { buy_ex, sell_ex } in spreads {
                        let (Some(buy_exchange), Some(sell_exchange)) =
                            (self.exchanges.get(&buy_ex), self.exchanges.get(&sell_ex))
                        else {
                            continue;
                        };
                        normalized.push((pair.clone(), buy_exchange, sell_exchange));
                    }
                    normalized
                })
                .flatten();

            let mut orderbook_futures: Vec<_> = normalized_pairs
                .into_iter()
                .map(|(pair, buy_exchange, sell_exchange)| async move {
                    let (buy_book, sell_book) = tokio::join!(
                        async {
                            match buy_exchange.orderbook(&pair.base, &pair.quote).await {
                                Ok(book) => Ok(book),
                                Err(e) => {
                                    error!(
                                        "Не удалось получить orderbook с {} - {}",
                                        buy_exchange.config().name,
                                        e
                                    );
                                    Err(())
                                }
                            }
                        },
                        async {
                            match sell_exchange
                                .futures_orderbook(&pair.base, &pair.quote)
                                .await
                            {
                                Ok(book) => Ok(book),
                                Err(e) => {
                                    error!(
                                        "Не удалось получить orderbook с {} - {}",
                                        sell_exchange.config().name,
                                        e
                                    );
                                    Err(())
                                }
                            }
                        }
                    );
                    (pair, buy_exchange, sell_exchange, buy_book, sell_book)
                })
                .collect();

            let books_time = Instant::now();

            let mut orderbook_results: Vec<(
                TradingPair,
                &Exchange,
                &Exchange,
                Result<OrderBook, ()>,
                Result<OrderBook, ()>,
            )> = Vec::new();

            // Выполняем чанками, забирая владение фьючеров
            while !orderbook_futures.is_empty() {
                let chunk_size = std::cmp::min(REQUESTS_CHUNK_SIZE, orderbook_futures.len());
                let chunk: Vec<_> = orderbook_futures.drain(0..chunk_size).collect();
                let chunk_results = join_all(chunk).await;
                orderbook_results.extend(chunk_results);
            }

            orderbook_total_elapsed += books_time.elapsed().as_nanos();

            let orderbook_passed_pairs: Vec<(
                TradingPair,
                &Exchange,
                &Exchange,
                OrderBook,
                OrderBook,
            )> = orderbook_results
                .into_iter()
                .filter_map(
                    |(
                        pair,
                        buy_exchange,
                        sell_exchange,
                        buy_orderbook_result,
                        sell_orderbook_result,
                    )| {
                        match (buy_orderbook_result, sell_orderbook_result) {
                            (Ok(buy_orderbook), Ok(sell_orderbook)) => Some((
                                pair,
                                buy_exchange,
                                sell_exchange,
                                buy_orderbook,
                                sell_orderbook,
                            )),
                            _ => None,
                        }
                    },
                )
                .collect();
            orderbook_passed += orderbook_passed_pairs.len();

            let mut spread_pairs: HashMap<TradingPair, Vec<SpreadBundle>> = HashMap::new();

            for (pair, buy_exchange, sell_exchange, buy_orderbook, sell_orderbook) in
                orderbook_passed_pairs
            {
                // Получение объема квота
                let Some(quote_volume) = pair.get_volume_by_quote() else {
                    error!("Не удалось получить объем квота {:?}", pair);
                    continue;
                };
                // Просчет ордербук цен
                let calc_books_time = Instant::now();
                let (calculated_asks, calculated_bids) = (
                    calculate_price_by_glass(&(quote_volume * QUOTE_RATE), &buy_orderbook.0),
                    calculate_price_by_glass(&(quote_volume * QUOTE_RATE), &sell_orderbook.1),
                );

                calc_books_total_elapsed += calc_books_time.elapsed().as_nanos();
                let (Some(calculated_buy_price), Some(calculated_sell_price)) =
                    (calculated_asks, calculated_bids)
                else {
                    continue;
                };
                calc_books_passed += 1;

                // Вычисление спреда
                let orderbook_spread_time = Instant::now();
                let orderbook_spread =
                    comput_spread_percent(&calculated_buy_price, &calculated_sell_price);
                orderbook_spread_total_elapsed += orderbook_spread_time.elapsed().as_nanos();

                if orderbook_spread < REQUIRED_ORDERBOOK_SPREAD_PERCENT || orderbook_spread > 10.0 {
                    continue;
                }
                orderbook_spread_passed += 1;

                let base_profit = quote_volume * (orderbook_spread / 100.0);
                let spread_pair = SpreadBundle {
                    pair: pair.clone(),
                    buy_ex: buy_exchange,
                    sell_ex: sell_exchange,
                    buy_price: calculated_buy_price,
                    sell_price: calculated_sell_price,
                    base_profit: base_profit,
                    quote_volume: quote_volume,
                };

                if let Some(spread_list) = spread_pairs.get_mut(&pair) {
                    spread_list.push(spread_pair);
                } else {
                    spread_pairs.insert(pair, vec![spread_pair]);
                };
            }

            for spread_pair in spread_pairs {
                let mut message = format!(" ✅ {}/{}", spread_pair.0.base, spread_pair.0.quote);
                let horizontal_line = format!("{}", "─".repeat(156));

                message += &("\n┌".to_owned() + &horizontal_line + "┐\n");
                message += &format!(
                    "│ {:<9} │ {:<9} │ {:<10} │ {:<12} │ {:<12} │ {:<13} │",
                    "Buy Ex", "Sell Ex", "Quote Vol.", "Buy Price", "Sell Price", "Base Profit",
                );
                for bundle in spread_pair.1 {
                    let bundle_line = format!(
                        "│ {:<9} │ {:<9} │ {:<10} │ {:<12.5} │ {:<12.5} │ {:<13.2} │",
                        bundle.buy_ex.config().name,
                        bundle.sell_ex.config().name,
                        bundle.quote_volume,
                        bundle.buy_price,
                        bundle.sell_price,
                        bundle.base_profit,
                    );

                    message += &("\n├".to_owned() + &horizontal_line + "┤\n");
                    message += &bundle_line;
                }

                info!(target: "info_module", "\n{}\n", message);
            }

            loop_elapsed += start_time.elapsed().as_nanos();

            loop_count += 1;
            if loop_count == loop_to_update_stat {
                // let formated_total_prices = format_duration(prices_quard_total_elapsed);
                let formated_total_ticker = format_duration(ticker_spread_total_elapsed);
                let formated_total_orderbook = format_duration(orderbook_total_elapsed);
                let formated_total_calc_books = format_duration(calc_books_total_elapsed);
                let formated_total_final = format_duration(orderbook_spread_total_elapsed);
                let formated_total_loop = format_duration(loop_elapsed);

                // "│ ├─ └─"
                stat_info += &format!("├──── Ticker spread stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_ticker);
                stat_info += &format!("│        └─ Total passed: {}\n", ticker_spread_passed);
                stat_info += &format!("├──── Orderbook stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_orderbook);
                stat_info += &format!("│        └─ Total passed: {}\n", orderbook_passed);
                stat_info += &format!("├──── Calculated Books stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_calc_books);
                stat_info += &format!("│        └─ Total passed: {}\n", calc_books_passed);
                stat_info += &format!("├──── Final SPOT-FUTURES spread stat\n");
                stat_info += &format!("│        ├─ Total elapsed: {}\n", formated_total_final);
                stat_info += &format!("│        └─ Total passed: {}\n", orderbook_spread_passed);
                stat_info += &format!("└── Total elapsed: {}\n", formated_total_loop);

                info!(target: "info_module", "{}", stat_info);

                (
                    loop_count,
                    ticker_spread_passed,
                    ticker_spread_total_elapsed,
                    orderbook_passed,
                    orderbook_total_elapsed,
                    calc_books_passed,
                    calc_books_total_elapsed,
                    orderbook_spread_passed,
                    orderbook_spread_total_elapsed,
                    loop_elapsed,
                ) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

                stat_info = format!(
                    "┌── СomputWorker:{} Statistics for {} iteration\n",
                    self.id, loop_to_update_stat
                );
            }
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }
    }
}
