use std::sync::Arc;

use arbitrage_system::{
    core::{traits::Workable, types::TradingPairs},
    init::exchanges::get_exchanges,
    workers::{comput_worker::ComputWorker, ticker_worker::TickerWorker},
};
use tokio::sync::RwLock;

// use arbitrage_system::{
//     // config::PAIRS_PER_THREAD, core::types::trading_pair::{PriceData, TradingPair}, init::exchanges::get_exchanges
//     // core::{net::websocket::WebSocketClient, utils::print_dashboard},
//     // init::{exchanges::get_exchanges, trading_pairs::get_trading_pairs},
//     // workers::{FetchWorker, comput_worker::ComputWorker, ticker_worker::TickerWorker},
// };

#[tokio::main]
async fn main() {
    let exchanges = Arc::new(get_exchanges().await);
    // let trading_pairs = get_trading_pairs();
    let trading_pairs = Arc::new(RwLock::new(TradingPairs::new()));

    let spread_pairs = Arc::new(RwLock::new(TradingPairs::new()));

    let comput_exchanges = exchanges.clone();
    let comput_trading_pairs = trading_pairs.clone();
    let mut tasks = Vec::new();

    let ticker_task = tokio::spawn(async move {
        let worker = TickerWorker::new(0, trading_pairs, exchanges);
        worker.run().await;
    });

    let comput_task = tokio::spawn(async move {
        let computer = ComputWorker::new(1, comput_trading_pairs, spread_pairs, comput_exchanges);
        computer.run().await;
    });

    tasks.extend([ticker_task, comput_task]);

    // let mut tasks = Vec::new();

    // let exchanges_clone = exchanges.clone();
    // let trading_pairs_clone = trading_pairs.clone();

    // let explorer_task = tokio::spawn(async move {
    //     loop {
    //         print_dashboard(trading_pairs_clone.clone(), exchanges_clone.clone());
    //         tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
    //     }
    // });
    // tasks.push(explorer_task);

    // let mut i = 0;

    // while i < trading_pairs.len() {
    //     let fetchers_exchanges = exchanges.clone();
    //     let comput_exchanges = exchanges.clone();

    //     let end = std::cmp::min(i + PAIRS_PER_THREAD, trading_pairs.len());

    //     let fetchers_pairs = trading_pairs[i..end].to_vec();
    //     let comput_pairs = trading_pairs[i..end].to_vec();

    //     let fetcher_task = tokio::spawn(async move {
    //         let fetcher = FetchWorker::new(
    //             i as usize / PAIRS_PER_THREAD,
    //             fetchers_pairs,
    //             fetchers_exchanges,
    //         );
    //         fetcher.run().await;
    //     });
    //     let comput_task = tokio::spawn(async move {
    //         let comput = ComputWorker::new(
    //             i as usize / PAIRS_PER_THREAD,
    //             comput_pairs,
    //             comput_exchanges,
    //         );
    //         comput.run().await;
    //     });

    //     tasks.push(fetcher_task);
    //     tasks.push(comput_task);

    //     i += PAIRS_PER_THREAD;
    // }
    // println!(
    //     "\nКоличество потоков: {}\nВ каждом потоке до {} торговых пар",
    //     tasks.len() - 1,
    //     PAIRS_PER_THREAD
    // );

    for task in tasks {
        task.await.unwrap();
    }
}
