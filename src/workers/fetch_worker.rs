use std::{sync::Arc, time::Instant};

use crate::{
    config::{USDT_LIMIT, exchanges},
    core::{
        traits::{Exchange, Tradeble},
        types::trading_pair::TradingPair,
        utils::calculate_price_by_glass,
    },
};

pub struct FetchWorker {
    id: usize,
    trading_pairs: Vec<Arc<TradingPair>>,
    exchanges: Vec<Arc<dyn Exchange>>,
}

impl FetchWorker {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn new(
        id: usize,
        trading_pairs: Vec<Arc<TradingPair>>,
        exchanges: Vec<Arc<dyn Exchange>>,
    ) -> Self {
        Self {
            id,
            trading_pairs,
            exchanges,
        }
    }
    pub async fn run(&self) {
        loop {
            let start = Instant::now();

            for exchange in &self.exchanges {
                let tickets = exchange.fetch_tickets().await;
            }
            let duration = start.elapsed();
            println!(
                "Фетч воркер id {} - закончил свою работу за {:?}",
                self.id(),
                duration
            );
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}

//  pub async fn run(&self) {
//         loop {
//             let start = Instant::now();
//             for pair in &self.trading_pairs {
//                 let pair_time = Instant::now();

//                 for (i, exchange) in self.exchanges.iter().enumerate() {
//                     if exchange.is_pair_excluded(pair.base(), pair.quote()) {
//                         continue;
//                     };

//                     let orderbook = match exchange.fetch_orderbook(pair.base(), pair.quote()).await
//                     {
//                         Ok(book) => book,
//                         Err(e) => {
//                             eprintln!(
//                                 "Ошибка получения стакана с {} для {}/{} - {}",
//                                 exchange.name(),
//                                 pair.base(),
//                                 pair.quote(),
//                                 e
//                             );
//                             continue;
//                         }
//                     };
//                     let buy_price = match calculate_price_by_glass(&USDT_LIMIT, &orderbook.0) {
//                         Some(price) => price,
//                         None => {
//                             // eprintln!(
//                             //     "Не удалось посчитать среднюю цену покупки {}/{} на {} - возможно не хватает объема в стакане",
//                             //     pair.base(),
//                             //     pair.quote(),
//                             //     exchange.name(),
//                             // );
//                             continue;
//                         }
//                     };
//                     let sell_price = match calculate_price_by_glass(&USDT_LIMIT, &orderbook.1) {
//                         Some(price) => price,
//                         None => {
//                             // eprintln!(
//                             //     "Не удалось посчитать среднюю цену продажи {}/{} на {} - возможно не хватает объема в стакане",
//                             //     pair.base(),
//                             //     pair.quote(),
//                             //     exchange.name(),
//                             // );
//                             continue;
//                         }
//                     };
//                     pair.update_buy_price(i, buy_price);
//                     pair.update_sell_price(i, sell_price);
//                 }
//                 let pair_time_elapsed = pair_time.elapsed();
//                 println!(
//                     "Фетч воркер id {} - обработал пару {} за {:?}",
//                     self.id(),
//                     pair.base(),
//                     pair_time_elapsed
//                 );
//             }
//             let duration = start.elapsed();
//             println!(
//                 "Фетч воркер id {} - закончил свою работу за {:?}",
//                 self.id(),
//                 duration
//             );
//             tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
//         }
//     }
