use crate::core::{
    traits::Workable,
    types::{Exchanges, TradingPairs},
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

                let (Some(buy_price), Some(sell_price)) = (
                    &*trading_pair.1.min_buy_price.read().await,
                    &*trading_pair.1.max_sell_price.read().await,
                ) else {
                    continue;
                };

                let spread = (sell_price.1 / buy_price.1 - 1.0) * 100.0;

                if spread < 0.5 {
                    continue;
                }

                println!(
                    "{}/{}. spread is {:.2}% buy: {} sell: {}",
                    base, quote, spread, buy_price.0, sell_price.0
                );

                // let_buy_book = buy_price.0
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
