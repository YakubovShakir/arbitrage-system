// use std::sync::Arc;

// use crate::core::{
//     traits::{Exchange, Tradeble, Workable},
//     types::trading_pair::TradingPair,
// };

// pub struct ComputWorker {
//     id: usize,
//     trading_pairs: Vec<Arc<TradingPair>>,
//     exchanges: Vec<Arc<dyn Exchange>>,
// }

// impl ComputWorker {
//     pub fn new(
//         id: usize,
//         trading_pairs: Vec<Arc<TradingPair>>,
//         exchanges: Vec<Arc<dyn Exchange>>,
//     ) -> Self {
//         Self {
//             id,
//             trading_pairs,
//             exchanges,
//         }
//     }
// }

// impl Workable for ComputWorker {
//     fn id(&self) -> usize {
//         self.id
//     }

//     async fn run(&self) -> ! {
//         loop {
//             for pair in &self.trading_pairs {
//                 let buy_price = match pair.min_buy_price() {
//                     Some((_, price)) => price,
//                     None => break,
//                 };
//                 let sell_price = match pair.max_sell_price() {
//                     Some((_, price)) => price,
//                     None => break,
//                 };

//                 let buy_exchange = match pair.min_buy_price() {
//                     Some((exchange, _)) => exchange,
//                     None => 0,
//                 };
//                 let sell_exchange = match pair.max_sell_price() {
//                     Some((exchange, _)) => exchange,
//                     None => 0,
//                 };

//                 let spread = (sell_price / buy_price - 1.0) * 100.0;

//                 if spread >= 1.0 && spread <= 40.0 {
//                     println!(
//                         "{}/{} Buy on {} Sell on {}. Spread = {:.2}",
//                         pair.base(),
//                         pair.quote(),
//                         self.exchanges[buy_exchange].name(),
//                         self.exchanges[sell_exchange].name(),
//                         spread,
//                     )
//                 }
//             }
//             // tokio::time::sleep(std::time::Duration::from_millis(12000)).await;
//         }
//     }
// }
