use arbitrage_system::{
    config::parameters::{INFO_CODE, RESET_CODE},
    core::{
        traits::exchange_service::{
            MarginInfoService, NetworkService, OrderBookService, TickerService,
        },
        types::{API, TradingPair},
    },
    init::{exchanges::get_exchanges, load_env::load_env},
};

#[tokio::test]
async fn test_ticker_service() -> Result<(), Box<dyn std::error::Error>> {
    load_env();
    let _ = log4rs::init_file("./src/config/log4rs.yaml", Default::default());

    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;
        let mut tickers_count = 0;

        while attempt <= attempts {
            match exchange.tickers().await {
                Ok(tickers) => {
                    if tickers.len() == 0 {
                        println!(
                            "❗ {} attempt failed for {} - empty tickers . Retry after 2 secs.",
                            attempt, exchange_name,
                        );
                        attempt += 1;
                        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                    } else {
                        tickers_count = tickers.len();
                        break;
                    }
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
    load_env();
    let _ = log4rs::init_file("./src/config/log4rs.yaml", Default::default());

    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;

        while attempt <= attempts {
            match exchange.orderbook("BTC", "USDT").await {
                Ok(orderbook) => {
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
                    // println!("{:#?}", orderbook);
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
    load_env();
    let _ = log4rs::init_file("./src/config/log4rs.yaml", Default::default());

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
    load_env();
    let _ = log4rs::init_file("./src/config/log4rs.yaml", Default::default());

    let exchanges = get_exchanges()?;

    for (exchange_name, exchange) in exchanges {
        let mut attempt = 1;
        let attempts = 5;
        let mut withdraw_networks_count = 0;
        let mut deposit_networks_count = 0;

        while attempt <= attempts {
            match exchange.networks("USDT").await {
                Ok(networks) => {
                    assert!(
                        networks.0.len() + networks.1.len() > 0,
                        "❗ Empty networks from {}",
                        exchange_name
                    );
                    println!("{:#?}", networks);
                    withdraw_networks_count = networks.0.len();
                    deposit_networks_count = networks.1.len();
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
                "❌ Cannot get networks from {} for {} attempts",
                exchange_name, attempts
            );
        } else {
            println!(
                "{INFO_CODE}[INFO] {} networks\nWithdraw: {}\nDeposit: {}\nAttempts: {}{RESET_CODE}",
                exchange_name, withdraw_networks_count, deposit_networks_count, attempt
            );
        }
    }

    Ok(())
}
