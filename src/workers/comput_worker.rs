use crate::{
    config::{USDT_LIMIT, parameters::REQUIRED_SPREAD_PERCENT},
    core::{
        traits::{Workable, exchange_service::OrderBookService},
        types::{Exchanges, TradingPairs},
        utils::{calculate_price_by_glass, comput_spread_percent},
    },
};
use std::{sync::Arc, time::Instant};

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

impl Workable for ComputWorker {
    fn id(&self) -> usize {
        self.id
    }

    async fn run(&self) -> ! {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            let start = Instant::now();

            for entry in self.trading_pairs.iter() {
                let pair = entry.key();
                let price_data = entry.value();

                if self.spread_pairs.contains_key(pair) {
                    continue;
                }
                if price_data.check_if_not_checked().await {
                    continue;
                }

                let (buy_price, sell_price) = {
                    let buy_guard = price_data.min_buy_price.read().await;
                    let sell_guard = price_data.max_sell_price.read().await;
                    (buy_guard, sell_guard)
                };

                // Вычисление тикер-спреда
                let spread = comput_spread_percent(&buy_price.1, &sell_price.1);
                if spread < REQUIRED_SPREAD_PERCENT {
                    continue;
                }

                let (buy_exchange, sell_exchange) = match (
                    self.exchanges.get(&buy_price.0),
                    self.exchanges.get(&sell_price.0),
                ) {
                    (Some(buy), Some(sell)) => (buy, sell),
                    _ => continue,
                };

                // let Some(networks) =
                //     verify_arbitrage_conditions_and_get_networks(buy_exchange, sell_exchange, pair)
                //         .await
                // else {
                //     continue;
                // };

                let (buy_book, sell_book) = tokio::join!(
                    async {
                        match buy_exchange.orderbook(&pair.base, &pair.quote).await {
                            Ok(book) => Ok(book),
                            Err(_) => {
                                // eprintln!("Buy orderbook error: {}", e);
                                Err(()) // Просто возвращаем ошибку без деталей
                            }
                        }
                    },
                    async {
                        match sell_exchange.orderbook(&pair.base, &pair.quote).await {
                            Ok(book) => Ok(book),
                            Err(_) => {
                                // eprintln!("Sell orderbook error: {}", e);
                                Err(())
                            }
                        }
                    }
                );

                let (Ok(buy_book), Ok(sell_book)) = (buy_book, sell_book) else {
                    continue;
                };

                let Some(buy_price_from_book) = calculate_price_by_glass(&USDT_LIMIT, &buy_book.0)
                else {
                    continue;
                };
                let Some(sell_price_from_book) =
                    calculate_price_by_glass(&USDT_LIMIT, &sell_book.1)
                else {
                    continue;
                };

                let final_spread = (sell_price_from_book / buy_price_from_book - 1.0) * 100.0;
                if final_spread < REQUIRED_SPREAD_PERCENT || final_spread > 10.0 {
                    continue;
                }

                // Попробовать использовать dash-set
                // self.spread_pairs.insert(pair.clone(), price_data.clone());

                println!(
                    "✅ {}/{}. spread {:.2}% buy: {} sell: {}",
                    pair.base,
                    pair.quote,
                    final_spread,
                    buy_exchange.config().name,
                    sell_exchange.config().name,
                    // networks
                );
            }
            let elapsed = start.elapsed();

            println!(
                "Просмотр {} торговых пар занял {:?}\n",
                self.trading_pairs.len(),
                elapsed
            );
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }
    }
}
