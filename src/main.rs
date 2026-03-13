use arbitrage_system::{
    // config::parameters::EXCHANGES_PER_TICKER_THREAD,
    core::{traits::Workable, types::TradingPairs},
    init::{exchanges::get_exchanges, load_env::load_env},
    workers::{comput_worker::ComputWorker, ticker_worker::TickerWorker},
};

use std::sync::Arc;
#[tokio::main]
async fn main() {
    log4rs::init_file("./src/config/log4rs.yaml", Default::default()).unwrap();

    load_env();
    // Получаем список бирж из init
    let exchanges = Arc::new(get_exchanges().unwrap());

    // Определяем список торговых пар и спред-пар
    let trading_pairs = Arc::new(TradingPairs::new());
    let spread_pairs = Arc::new(TradingPairs::new());

    // let tickers_produced: Vec<Arc<TradingPairs>> = Vec::from(Arc::new(TradingPairs::new()));
    let mut tasks = Vec::new();
    // ------------
    let comput_exchanges = exchanges.clone();
    let comput_trading_pairs = trading_pairs.clone();

    let ticker_task = tokio::spawn(async move {
        let worker = TickerWorker::new(0, trading_pairs, exchanges);
        worker.run().await;
    });

    // let comput_task = tokio::spawn(async move {
    //     let computer: ComputWorker =
    //         ComputWorker::new(1, comput_trading_pairs, spread_pairs, comput_exchanges);
    //     computer.run().await;
    // });

    tasks.extend([ticker_task]);

    for task in tasks {
        task.await.unwrap();
    }
}
