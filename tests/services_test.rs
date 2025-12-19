use arbitrage_system::{
    core::{
        traits::exchange_service::{
            MarginInfoService, NetworkService, OrderBookService, TickerService,
        },
        types::{API, TradingPair},
    },
    init::exchanges::get_exchanges,
};

#[tokio::test]
async fn test_ticker_service() -> Result<(), Box<dyn std::error::Error>> {
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

#[tokio::test]
async fn test_orderbook_service() -> Result<(), Box<dyn std::error::Error>> {
    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;

        while attempt <= attempts {
            if let Ok(orderbook) = exchange.orderbook("BTC", "USDT").await {
                assert!(
                    orderbook.0.len() > 0,
                    "❗ Empty ASKS from {}",
                    exchange_name
                );
                assert!(
                    orderbook.1.len() > 0,
                    "❗ Empty BIDS tickers from {}",
                    exchange_name
                );
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
                "❌ Cannot get orderbook from {} for {} attempts",
                exchange_name, attempts
            );
        } else {
            println!(
                "🟢 Receive orderbook {} for {} attempt",
                exchange_name, attempt
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_margin_info_service() -> Result<(), Box<dyn std::error::Error>> {
    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let Some(_) = API::GetMarginInfo.endpoint(&exchange) else {
            continue;
        };
        let mut attempt = 1;
        let attempts = 5;

        while attempt <= attempts {
            match exchange.borrowable(&TradingPair::new("BTC", "USDT")).await {
                Ok(borrowable) => {
                    // assert!(tickers.len() > 0, "❗ Empty tickers from {}", exchange_name);

                    println!("{} {}", exchange_name, borrowable);
                    break;
                }
                Err(e) => {
                    println!(
                        "❗ {} attempt failed for {} - {}. Retry after 2 secs.",
                        attempt, exchange_name, e
                    );
                    attempt += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                }
            }
        }
        if attempt > attempts {
            panic!(
                "❌ Cannot get borrowable status from {} for {} attempts",
                exchange_name, attempts
            );
        } else {
            println!(
                "🟢 Receive borrowable status from {} for {} attempt",
                exchange_name, attempt
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_network_service() -> Result<(), Box<dyn std::error::Error>> {
    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;
        let mut networks_count = 0;

        while attempt <= attempts {
            if let Ok(networks) = exchange.networks("BTC").await {
                assert!(
                    networks.len() > 0,
                    "❗ Empty networks from {}",
                    exchange_name
                );
                networks_count = networks.len();
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
                "❌ Cannot get networks from {} for {} attempts",
                exchange_name, attempts
            );
        } else {
            println!(
                "🟢 Receive {} networks from {} for {} attempt",
                networks_count, exchange_name, attempt
            );
        }
    }

    Ok(())
}
