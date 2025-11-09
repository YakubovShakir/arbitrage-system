use std::{collections::HashMap, sync::Arc};

use crate::core::{
    traits::{Exchange, Workable},
    types::trading_pair::{PriceData, TradingPair},
};

pub struct TickerWorker {
    id: usize,
    // trading_pairs: HashMap<TradingPair, Arc<PriceData>>,
    exchanges: Vec<Arc<dyn Exchange>>,
}

impl TickerWorker {
    pub fn new(
        id: usize,
        // trading_pairs: HashMap<TradingPair, Arc<PriceData>>,
        exchanges: Vec<Arc<dyn Exchange>>,
    ) -> Self {
        Self {
            id,
            // trading_pairs,
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
            for exchange in self.exchanges.clone() {
                let tickers = exchange.fetch_tickers().await;

                println!("{:#?}", tickers);
            }
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}
