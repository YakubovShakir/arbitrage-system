use futures_util::future::join_all;

use crate::{
    config::parameters::{
        QUOTE_RATE, REQUESTS_CHUNK_SIZE, REQUIRED_ORDERBOOK_SPREAD_PERCENT,
        REQUIRED_TICKER_SPREAD_PERCENT, RESET_CODE,
    },
    core::{
        traits::{Workable, exchange_service::OrderBookService},
        types::{
            Exchanges, OrderBook, TradingPair, TradingPairs,
            blacklist::Blacklist,
            exchanges::Exchange,
            network::{DepositNetwork, WithdrawNetwork},
            trading_pair,
        },
        utils::{
            calculate_price_by_glass, comput_spread_percent, format_duration,
            verify_arbitrage_conditions_and_get_networks,
        },
    },
};
use log::{error, info};
use std::{collections::HashMap, sync::Arc, time::Instant};

struct TickerSpread<'a> {
    pair: TradingPair,
    buy_ex: &'a Exchange,
    sell_ex: &'a Exchange,
}

struct SpreadBundle<'a> {
    pair: TradingPair,
    buy_ex: &'a Exchange,
    sell_ex: &'a Exchange,
    networks: Vec<(WithdrawNetwork, DepositNetwork)>,
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
    blacklist: Arc<Blacklist>,
}

impl ComputWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<TradingPairs>,
        spread_pairs: Arc<TradingPairs>,
        exchanges: Arc<Exchanges>,
        blacklist: Arc<Blacklist>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            spread_pairs,
            exchanges,
            blacklist,
        }
    }
}
impl ComputWorker {
    async fn find_ticker_spreads(&self) -> (Vec<TickerSpread>, u128) {
        let mut ticker_spreads: Vec<TickerSpread> = Vec::new();
        let mut comput_elapsed: u128 = 0;
        let comput_time = Instant::now();

        for entry in self.trading_pairs.iter() {
            let buy_list = &entry.buy_price_list;
            let sell_list = &entry.sell_price_list;

            for buy_item in buy_list {
                let buy_checked = buy_item.check().await;

                if buy_checked {
                    for sell_item in sell_list {
                        if buy_item.exchange == sell_item.exchange {
                            continue;
                        }

                        let spread = comput_spread_percent(
                            &buy_item.get_price().await,
                            &sell_item.get_price().await,
                        );

                        if spread < REQUIRED_TICKER_SPREAD_PERCENT {
                            continue;
                        }

                        let (buy_exchange, sell_exchange) = {
                            match (
                                self.exchanges.get(&buy_item.exchange),
                                self.exchanges.get(&sell_item.exchange),
                            ) {
                                (Some(buy), Some(sell)) => (buy, sell),
                                _ => continue,
                            }
                        };

                        ticker_spreads.push(TickerSpread {
                            pair: entry.key().clone(),
                            buy_ex: &buy_exchange,
                            sell_ex: &sell_exchange,
                        });
                    }
                } else {
                    for sell_item in sell_list {
                        let sell_checked = sell_item.check().await;

                        if buy_item.exchange == sell_item.exchange || !sell_checked {
                            continue;
                        }

                        let spread = comput_spread_percent(
                            &buy_item.get_price().await,
                            &sell_item.get_price().await,
                        );

                        if spread < REQUIRED_TICKER_SPREAD_PERCENT {
                            continue;
                        }

                        let (buy_exchange, sell_exchange) = {
                            match (
                                self.exchanges.get(&buy_item.exchange),
                                self.exchanges.get(&sell_item.exchange),
                            ) {
                                (Some(buy), Some(sell)) => (buy, sell),
                                _ => continue,
                            }
                        };

                        ticker_spreads.push(TickerSpread {
                            pair: entry.key().clone(),
                            buy_ex: &buy_exchange,
                            sell_ex: &sell_exchange,
                        });
                    }
                }
            }
        }
        comput_elapsed += comput_time.elapsed().as_nanos();

        (ticker_spreads, comput_elapsed)
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

        let mut verify_passed = 0;
        let mut verify_total_elapsed: u128 = 0;

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
            let (pairs, elapsed) = self.find_ticker_spreads().await;
            ticker_spread_total_elapsed += elapsed;
            ticker_spread_passed += pairs.len();

            // Параллельное выполнение верификации
            let verify_futures: Vec<_> = pairs
                .into_iter()
                .map(
                    |TickerSpread {
                         pair,
                         buy_ex,
                         sell_ex,
                     }| async move {
                        match verify_arbitrage_conditions_and_get_networks(&buy_ex, &sell_ex, &pair)
                            .await
                        {
                            Ok(networks) => {
                                if networks.len() != 0 {
                                    Some((pair, buy_ex, sell_ex, networks))
                                } else {
                                    None
                                }
                            }
                            Err(failed_exchange) => {
                                self.trading_pairs.remove(&pair);
                                if failed_exchange == buy_ex.config().name {
                                    self.blacklist.blacklist_buy(&pair, failed_exchange);
                                } else {
                                    self.blacklist.blacklist_sell(&pair, failed_exchange);
                                }
                                None
                            }
                        }
                    },
                )
                .collect();

            let verify_time = Instant::now();
            let verify_results: Vec<
                Option<(
                    TradingPair,
                    &Exchange,
                    &Exchange,
                    Vec<(WithdrawNetwork, DepositNetwork)>,
                )>,
            > = join_all(verify_futures).await;
            verify_total_elapsed += verify_time.elapsed().as_nanos();

            let verify_passed_pairs: Vec<(
                TradingPair,
                &Exchange,
                &Exchange,
                Vec<(WithdrawNetwork, DepositNetwork)>,
            )> = verify_results
                .into_iter()
                .filter_map(|result| result)
                .collect();
            verify_passed += verify_passed_pairs.len();

            let mut orderbook_futures: Vec<_> = verify_passed_pairs
                .into_iter()
                .map(|(pair, buy_exchange, sell_exchange, networks)| async move {
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
                            match sell_exchange.orderbook(&pair.base, &pair.quote).await {
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
                    (
                        pair,
                        buy_exchange,
                        sell_exchange,
                        networks,
                        buy_book,
                        sell_book,
                    )
                })
                .collect();

            let books_time = Instant::now();

            let mut orderbook_results: Vec<(
                TradingPair,
                &Exchange,
                &Exchange,
                Vec<(WithdrawNetwork, DepositNetwork)>,
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
                Vec<(WithdrawNetwork, DepositNetwork)>,
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

            let mut spread_pairs: HashMap<TradingPair, Vec<SpreadBundle>> = HashMap::new();

            for (pair, buy_exchange, sell_exchange, networks, buy_orderbook, sell_orderbook) in
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
                    networks,
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
                let mut message = format!("✅ {}/{} ✅", spread_pair.0.base, spread_pair.0.quote);
                let horizontal_line = format!("{}", "─".repeat(175));

                message += &("\n┌".to_owned() + &horizontal_line + "┐\n");
                message += &format!(
                    "│ {:<9} │ {:<9} │ {:<8} │ {:<8} │ {:<8} │ {:<8} │ {:<60} │",
                    "Buy Ex",
                    "Sell Ex",
                    "Quote Volume",
                    "Buy Price",
                    "Sell Price",
                    "Base Profit",
                    "Networks"
                );
                for bundle in spread_pair.1 {
                    let mut network_message = format!("");

                    for (withdraw_network, deposit_network) in bundle.networks {
                        let fee_quote_volume =
                            withdraw_network.withdraw_fee.unwrap_or(0.0) * bundle.sell_price;
                        let profit_with_fee = bundle.base_profit - fee_quote_volume;
                        let final_spread_percent = profit_with_fee / bundle.quote_volume * 100.0;

                        let fee_message = match withdraw_network.withdraw_fee {
                            Some(fee) => format!(
                                "{:.2} {} ~ {:.2} {}",
                                fee, bundle.pair.base, fee_quote_volume, bundle.pair.quote
                            ),
                            None => String::from("∅"),
                        };
                        let num_of_conf_message = match deposit_network.number_of_confirmation {
                            Some(number) => number.to_string(),
                            None => String::from("∅"),
                        };
                        let contract_message = match withdraw_network
                            .base
                            .config
                            .contract_address
                            .or_else(|| deposit_network.base.config.contract_address)
                        {
                            Some(contract_address) => {
                                if contract_address.len() > 8 {
                                    let start = &contract_address[0..4];
                                    let end = &contract_address[contract_address.len() - 4..];
                                    format!("{}..{}", start, end)
                                } else {
                                    contract_address
                                }
                            }
                            None => String::from("∅"),
                        };

                        let message = format!(
                            "[{}|Fee:{}|C-ct.:{}|Conf-s.:{}|Income:{:.2}~{:.2}%]",
                            withdraw_network.base.network_type,
                            fee_message,
                            contract_message,
                            num_of_conf_message,
                            profit_with_fee,
                            final_spread_percent
                        );
                        network_message += &message;
                    }

                    let bundle_line = format!(
                        "│ {:<9} │ {:<9} │ {:<8} │ {:<8.5} │ {:<8.5} │ {:<8.2} │ {:<60} │",
                        bundle.buy_ex.config().name,
                        bundle.sell_ex.config().name,
                        bundle.quote_volume,
                        bundle.buy_price,
                        bundle.sell_price,
                        bundle.base_profit,
                        network_message
                    );

                    message += &("\n├".to_owned() + &horizontal_line + "\n");
                    message += &bundle_line;
                }

                info!(target: "info_module", "\n{}\n", message);
            }

            loop_elapsed += start_time.elapsed().as_nanos();

            loop_count += 1;
            if loop_count == loop_to_update_stat {
                // let formated_total_prices = format_duration(prices_quard_total_elapsed);
                let formated_total_ticker = format_duration(ticker_spread_total_elapsed);
                let formated_total_verify = format_duration(verify_total_elapsed);
                let formated_total_orderbook = format_duration(orderbook_total_elapsed);
                let formated_total_calc_books = format_duration(calc_books_total_elapsed);
                let formated_total_final = format_duration(orderbook_spread_total_elapsed);
                let formated_total_loop = format_duration(loop_elapsed);

                // "│ ├─ └─"
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
                stat_info += &format!("│        └─ Total passed: {}\n", orderbook_spread_passed);
                stat_info += &format!("└── Total elapsed: {}\n", formated_total_loop);

                info!(target: "info_module", "{}", stat_info);

                (
                    loop_count,
                    ticker_spread_passed,
                    ticker_spread_total_elapsed,
                    verify_passed,
                    verify_total_elapsed,
                    orderbook_passed,
                    orderbook_total_elapsed,
                    calc_books_passed,
                    calc_books_total_elapsed,
                    orderbook_spread_passed,
                    orderbook_spread_total_elapsed,
                    loop_elapsed,
                ) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

                stat_info = format!(
                    "┌── СomputWorker:{} Statistics for {} iteration\n",
                    self.id, loop_to_update_stat
                );
            }
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }
    }
}
