use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    core::{
        traits::Workable,
        types::{Exchanges, TradingPairs},
    },
    workers::ticker_worker,
};

pub struct TickerWorker {
    id: usize,
    trading_pairs: Arc<RwLock<TradingPairs>>,
    exchanges: Arc<Exchanges>,
}

impl TickerWorker {
    pub fn new(
        id: usize,
        trading_pairs: Arc<RwLock<TradingPairs>>,
        exchanges: Arc<Exchanges>,
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
            for (exchange_name, exchange) in &*self.exchanges {
                let new_tickers = match exchange.fetch_tickers().await {
                    Ok(tickers) => tickers,
                    Err(e) => {
                        println!(
                            "❌ Не получили тикеры от биржи {}, ошибка - {}",
                            exchange_name, e
                        );
                        continue;
                    }
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

                    let (buy_price, sell_price) = {
                        let buy_guard = new_price_data.min_buy_price.read().await;
                        let sell_guard = new_price_data.max_sell_price.read().await;
                        (buy_guard.1, sell_guard.1)
                    };

                    existing_price
                        .update_buy_price(exchange_name.clone(), buy_price)
                        .await;
                    existing_price
                        .update_sell_price(exchange_name.clone(), sell_price)
                        .await;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}
