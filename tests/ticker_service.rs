use arbitrage_system::{
    core::traits::exchange_service::TickerService, init::exchanges::get_exchanges,
};

// Define this in a crate called `adder`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[tokio::test]
async fn test_tickers() -> Result<(), Box<dyn std::error::Error>> {
    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;
        let mut tickers_count = 0;

        while attempt <= attempts {
            if let Ok(tickers) = exchange.tickers().await {
                assert!(tickers.len() > 0, "❗ Empty tickers from {}", exchange_name);
                tickers_count = tickers.len();
                break;
            } else {
                println!(
                    "❗ {} attempt failed for {}. Retry after 2 secs.",
                    attempt, exchange_name
                );
                attempt += 1;
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            }
        }

        if attempt > attempts {
            panic!(
                "❌ Cannot get tickers from {} for {} attempts",
                exchange_name, attempts
            );
        } else {
            println!(
                "🟢 Receive {} tickers from {} for {} attempt",
                tickers_count, exchange_name, attempt
            );
        }
    }

    Ok(())
}
