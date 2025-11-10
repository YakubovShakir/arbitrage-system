use crate::{
    core::{
        traits::{Exchange, Workable},
        types::trading_pair::{PriceData, TradingPair},
    },
    init::trading_pairs,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct TickerWorker {
    id: usize,
    trading_pairs: Arc<RwLock<HashMap<TradingPair, PriceData>>>,
    exchanges: Vec<Arc<dyn Exchange>>,
}

impl TickerWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<RwLock<HashMap<TradingPair, PriceData>>>,
        exchanges: Vec<Arc<dyn Exchange>>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            exchanges,
        }
    }
}

impl Workable for TickerWorker {
    fn id(&self) -> usize {
        self.id
    }

    async fn run(&self) -> ! {
        loop {
            println!(
                "Глобальная хеш-мапа торговых пар насчитывает {} пар",
                self.trading_pairs.read().await.len()
            );
            for exchange in &self.exchanges {
                let Some(new_tickers) = exchange.fetch_tickers().await else {
                    println!("❌ Не получили тикеры от биржи {}", exchange.name());
                    continue;
                };
                let mut global_pairs = self.trading_pairs.write().await;

                for (new_trading_pair, new_price_data) in new_tickers {
                    let Some(existing_price) = global_pairs.get(&new_trading_pair) else {
                        // println!(
                        //     "Пара {}/{} с биржи {} добавлена в глобальную хеш мапу",
                        //     new_trading_pair.base,
                        //     new_trading_pair.quote,
                        //     exchange.name(),
                        // );
                        global_pairs.insert(new_trading_pair, new_price_data);
                        continue;
                    };

                    if let Some((_, new_buy_price)) = new_price_data.min_buy_price() {
                        existing_price.update_buy_price(exchange.name().to_string(), new_buy_price);
                    }
                    if let Some((_, new_sell_price)) = new_price_data.max_sell_price() {
                        existing_price
                            .update_sell_price(exchange.name().to_string(), new_sell_price);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}
