use crate::{
    config::{USDT_LIMIT, parameters::REQUIRED_SPREAD_PERCENT},
    core::{
        traits::Workable,
        types::{Exchanges, TradingPairs},
        utils::{
            calculate_price_by_glass, comput_spread_percent,
            verify_arbitrage_conditions_and_get_networks,
        },
    },
};
use std::{sync::Arc, time::Instant};
use tokio::sync::RwLock;

pub struct ComputWorker {
    id: usize,
    trading_pairs: Arc<RwLock<TradingPairs>>,
    spread_pairs: Arc<RwLock<TradingPairs>>,
    exchanges: Arc<Exchanges>,
}

impl ComputWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<RwLock<TradingPairs>>,
        spread_pairs: Arc<RwLock<TradingPairs>>,
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
            let start = Instant::now();

            let global_pairs = self.trading_pairs.read().await;
            let global_spreads = self.spread_pairs.write().await;

            for trading_pair in &*global_pairs {
                if global_spreads.contains_key(trading_pair.0) {
                    continue;
                }

                let base = &trading_pair.0.base;
                let quote = &trading_pair.0.quote;

                // Получение тикер цену покупки и продажи
                let (Some(buy_price), Some(sell_price)) = (
                    &*trading_pair.1.min_buy_price.read().await,
                    &*trading_pair.1.max_sell_price.read().await,
                ) else {
                    continue;
                };

                // Вычисление тикер-спреда
                let mut spread = comput_spread_percent(&buy_price.1, &sell_price.1);
                if spread < REQUIRED_SPREAD_PERCENT {
                    continue;
                }

                let Some(buy_exchange) = self.exchanges.get(&buy_price.0) else {
                    continue;
                };
                let Some(sell_exchange) = self.exchanges.get(&sell_price.0) else {
                    continue;
                };

                let Some(networks) = verify_arbitrage_conditions_and_get_networks(
                    &buy_exchange,
                    &sell_exchange,
                    trading_pair.0,
                )
                .await
                else {
                    continue;
                };

                let Ok(buy_book) = buy_exchange.fetch_orderbook(&base, &quote).await else {
                    continue;
                };

                let Ok(sell_book) = sell_exchange.fetch_orderbook(&base, &quote).await else {
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
                spread = (sell_price_from_book / buy_price_from_book - 1.0) * 100.0;

                if spread < REQUIRED_SPREAD_PERCENT {
                    continue;
                }

                println!(
                    "{}/{}. spread is {:.2}% buy: {} sell: {} Networks {:?}",
                    base, quote, spread, buy_price.0, sell_price.0, networks
                );
            }

            let elapsed = start.elapsed();

            println!(
                "Просмотр {} торговых пар занял {:?}\n",
                global_pairs.len(),
                elapsed
            );
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}
