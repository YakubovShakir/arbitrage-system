use std::{sync::Arc, vec};

use arbitrage_system::{
    config::{self, PAIRS_PER_THREAD},
    exchanges::{Binance, Bybit, TradingPair, trading_pair},
    traits::ExchangeAPI,
    workers::FetchWorker,
};

#[tokio::main]
async fn main() {
    let binance = Arc::new(Binance::new(
        config::binance::NAME,
        config::binance::API_KEY,
        config::binance::SECRET_KEY,
        config::binance::BASE_URL,
        config::binance::AVAILABLE_PAIRS,
    ));

    let bybit = Arc::new(Bybit::new(
        config::bybit::NAME,
        config::bybit::API_KEY,
        config::bybit::SECRET_KEY,
        config::bybit::BASE_URL,
        config::bybit::AVAILABLE_PAIRS,
    ));

    let exchanges: Vec<Arc<dyn ExchangeAPI>> = vec![binance.clone(), bybit.clone()];
    let trading_pairs = Vec::from([
        Arc::new(TradingPair::new("AVAX", "USDT")),
        Arc::new(TradingPair::new("DOGE", "USDT")),
        Arc::new(TradingPair::new("ETH", "USDT")),
    ]);

    let mut tasks = Vec::new();
    let trading_pair_clone = trading_pairs.clone();

    let explorer_task = tokio::spawn(async move {
        loop {
            let trading_pair_clone = trading_pair_clone.clone();

            let mut status = format!(
                "\n{}\n| {:<14} | {:<14} | {:<14} | {:<14} | {:<14} |",
                "-".repeat(86),
                "TRADING PAIR",
                "BUY EXCHANGE",
                "BUY PRICE",
                "SELL EXCHANGE",
                "SELL PRICE"
            );
            status += &format!("\n{}\n", "-".repeat(86));

            for pair in trading_pair_clone {
                let pair_buy = match pair.min_buy_price() {
                    Some((exchange, price)) => (exchange, price),
                    None => ("-".to_string(), 0.0),
                };
                let pair_sell = match pair.max_sell_price() {
                    Some((exchange, price)) => (exchange, price),
                    None => ("-".to_string(), 0.0),
                };

                status += &format!(
                    "\n| {:<6}/{:<7} | {:<14} | {:<14.6} | {:<14} | {:<14.6} |",
                    pair.base().to_string(),
                    pair.quote().to_string(),
                    pair_buy.0,
                    pair_buy.1,
                    pair_sell.0,
                    pair_sell.1
                );
            }

            println!("{}", status);
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    });

    tasks.push(explorer_task);
    let mut i = 0;

    while i < trading_pairs.len() {
        let exchanges_clone = exchanges.clone();

        let end = std::cmp::min(i + PAIRS_PER_THREAD, trading_pairs.len());
        let pairs_clone = trading_pairs[i..end].to_vec();

        let task = tokio::spawn(async move {
            let fetcher = FetchWorker::new(i as i16 / 2, exchanges_clone, pairs_clone);
            fetcher.run().await;
        });
        tasks.push(task);
        i += PAIRS_PER_THREAD;
    }
    println!(
        "\nКоличество Фетчер потоков: {}\nВ каждом потоке до {} торговых пар",
        tasks.len() - 1,
        PAIRS_PER_THREAD
    );
    println!("Фетчер-воркеры запущены");

    for task in tasks {
        task.await.unwrap();
    }

    // let start = Instant::now();

    // let duration = start.elapsed();
}
