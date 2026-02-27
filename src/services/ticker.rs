use crate::{
    config::parameters::{GATE_TICKERS_HTTP_TIMEOUT_SECONDS, HUOBI_TICKERS_HTTP_TIMEOUT_SECONDS},
    core::{
        net::websocket::WebSocketClient,
        traits::exchange_service::TickerService,
        types::{API, TickerPrice, Tickers, TradingPair, exchanges::Exchange},
        utils::json_utils::{find_value_from_json_key, parse_json_as_f64},
    },
};
use async_trait::async_trait;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    time::Duration,
};

#[async_trait]
impl TickerService for Exchange {
    async fn tickers(&self) -> Result<Tickers, Box<dyn std::error::Error>> {
        // Checking is WebSocket interface available
        match &self.config().websocket_client {
            Some(ws_client) => Ok(handle_ws_interface(ws_client, self).await?),
            None => Ok(handle_http_interface(self).await?),
        }
    }
}

async fn handle_http_interface(exchange: &Exchange) -> Result<Tickers, Box<dyn std::error::Error>> {
    let Some(endpoint) = API::GetTickers.endpoint(exchange) else {
        return Err(format!(
            "Error: GetTickers endpoint is not set for the exchange {}",
            exchange.config().name
        )
        .into());
    };
    let mut trading_pairs: Tickers = HashMap::new();

    match exchange {
        Exchange::Bybit(cfg) => {
            let query = &[("category", "spot")];
            let response = cfg
                .http_client
                .get(endpoint, Some(query), None, None)
                .await?;
            let tickers = find_value_from_json_key(&response, &["result", "list"])?;
            for ticker in tickers.members() {
                let (Ok(parsed_ask_price), Ok(parsed_bid_price)) = (
                    parse_json_as_f64(&ticker["ask1Price"]),
                    parse_json_as_f64(&ticker["bid1Price"]),
                ) else {
                    continue;
                };

                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), parsed_ask_price),
                    sell_price: (cfg.name.to_owned(), parsed_bid_price),
                };

                if let Some(pair) = TradingPair::from_str(&ticker["symbol"].to_string()) {
                    trading_pairs.insert(pair, price);
                }
            }
        }

        Exchange::Bitget(cfg) => {
            // Просто запускаем, игнорируем ошибки соединения
            let response = cfg.http_client.get(endpoint, None, None, None).await?;
            let data = find_value_from_json_key(&response, &["data"])?;
            if data.is_empty() {
                return Err(format!("{} Invalid response: 'data' is empty array", cfg.name).into());
            }

            for ticker in data.members() {
                let (Ok(ask_price), Ok(bid_price)) = (
                    parse_json_as_f64(&ticker["askPr"]),
                    parse_json_as_f64(&ticker["bidPr"]),
                ) else {
                    continue;
                };

                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), ask_price),
                    sell_price: (cfg.name.to_owned(), bid_price),
                };

                if let Some(pair) = TradingPair::from_str(&ticker["symbol"].to_string()) {
                    trading_pairs.insert(pair, price);
                }
            }
        }
        Exchange::Gate(cfg) => {
            let headers = &[
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
            ];

            let tickers = cfg
                .http_client
                .get(
                    endpoint,
                    None,
                    Some(headers),
                    Some(Duration::from_secs(GATE_TICKERS_HTTP_TIMEOUT_SECONDS)),
                )
                .await?;
            if tickers.is_empty() {
                return Err(
                    format!("{} Invalid response: 'tickers' is empty array", cfg.name).into(),
                );
            }

            for ticker in tickers.members() {
                let (Ok(lowest_ask), Ok(highest_bid)) = (
                    parse_json_as_f64(&ticker["lowest_ask"]),
                    parse_json_as_f64(&ticker["highest_bid"]),
                ) else {
                    continue;
                };

                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), lowest_ask),
                    sell_price: (cfg.name.to_owned(), highest_bid),
                };
                if let Some(pair) =
                    TradingPair::from_str_with_separator(&ticker["currency_pair"].to_string(), '_')
                {
                    trading_pairs.insert(pair, price);
                }
            }
        }
        Exchange::Huobi(cfg) => {
            let headers = &[("Content-Type", "application/json")];

            let res = cfg
                .http_client
                .get(
                    endpoint,
                    None,
                    Some(headers),
                    Some(Duration::from_secs(HUOBI_TICKERS_HTTP_TIMEOUT_SECONDS)),
                )
                .await?;
            let tickers = find_value_from_json_key(&res, &["data"])?;

            for ticker in tickers.members() {
                let (Ok(ask_price), Ok(bid_price)) = (
                    parse_json_as_f64(&ticker["ask"]),
                    parse_json_as_f64(&ticker["bid"]),
                ) else {
                    continue;
                };
                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), ask_price),
                    sell_price: (cfg.name.to_owned(), bid_price),
                };
                if let Some(pair) = TradingPair::from_str(&ticker["symbol"].to_string()) {
                    trading_pairs.insert(pair, price);
                }
            }
        }
        Exchange::Bitmart(cfg) => {
            let res = cfg
                .http_client
                .get(
                    endpoint, None, None,
                    None, // Some(Duration::from_secs(HUOBI_TICKERS_HTTP_TIMEOUT_SECONDS)),
                )
                .await?;
            if res["message"] != "success" {
                return Err(format!(
                    "Cannot get tickers from {} because response.message doesn't 'success'",
                    cfg.name
                )
                .into());
            }
            for ticker in res["data"].members() {
                let (Ok(best_bid), Ok(best_ask)) = (
                    parse_json_as_f64(&ticker[8]),
                    parse_json_as_f64(&ticker[10]),
                ) else {
                    continue;
                };
                let price: TickerPrice = TickerPrice {
                    buy_price: (cfg.name.to_owned(), best_ask),
                    sell_price: (cfg.name.to_owned(), best_bid),
                };
                if let Some(trading_pair) =
                    TradingPair::from_str_with_separator(&ticker[0].to_string(), '_')
                {
                    trading_pairs.insert(trading_pair, price);
                }
            }
        }
        Exchange::Okx(cfg) => {
            let query = &[("instType", "SPOT")];
            let res = cfg
                .http_client
                .get(
                    endpoint,
                    Some(query),
                    None,
                    None, // Some(Duration::from_secs(HUOBI_TICKERS_HTTP_TIMEOUT_SECONDS)),
                )
                .await?;
            for item in res["data"].members() {
                let (Ok(best_bid), Ok(best_ask)) = (
                    parse_json_as_f64(&item["bidPx"]),
                    parse_json_as_f64(&item["askPx"]),
                ) else {
                    continue;
                };
                let price: TickerPrice = TickerPrice {
                    buy_price: (cfg.name.to_owned(), best_ask),
                    sell_price: (cfg.name.to_owned(), best_bid),
                };
                if let Some(trading_pair) =
                    TradingPair::from_str_with_separator(&item["instId"].to_string(), '-')
                {
                    trading_pairs.insert(trading_pair, price);
                }
            }
        }
        _ => {
            return Err(format!(
                "HTTP interface is not available for {}",
                exchange.config().name
            )
            .into());
        }
    }

    // if trading_pairs.is_empty() {
    //     return Err("Error: Trading Pairs is empty".into());
    // }

    Ok(trading_pairs)
}

async fn handle_ws_interface(
    client: &WebSocketClient,
    exchange: &Exchange,
) -> Result<Tickers, Box<dyn std::error::Error>> {
    let is_streamed_state = client.state_is_streamed;

    if is_streamed_state {
        if let Some(streamed_state) = client.get_streamed_state().await {
            return Ok(read_streamed_ws_state(&streamed_state, &exchange)?);
        }
    } else {
        if let Some(state) = client.get_state().await {
            return Ok(read_ws_state(&state, &exchange)?);
        }
    };

    // If any of state not readed -> establish connection
    let _ = establish_ws_connection_and_sub(client, exchange).await?;

    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;

    if is_streamed_state {
        if let Some(streamed_state) = client.get_streamed_state().await {
            return Ok(read_streamed_ws_state(&streamed_state, &exchange)?);
        }
    } else {
        if let Some(state) = client.get_state().await {
            return Ok(read_ws_state(&state, &exchange)?);
        }
    };

    Ok(Tickers::new())
}

fn read_streamed_ws_state(
    state_vec: &VecDeque<String>,
    exchange: &Exchange,
) -> Result<Tickers, Box<dyn std::error::Error>> {
    let mut trading_pairs: Tickers = HashMap::new();

    match exchange {
        Exchange::Kucoin(cfg) => {
            for item in state_vec {
                let parsed_ticker = json::parse(&item).or(Err(format!(
                    "Cannot parse ticker  from {} to JSON",
                    cfg.name
                )))?;

                let (Ok(parsed_buy_price), Ok(parsed_sell_price)) = (
                    parse_json_as_f64(&parsed_ticker["data"]["bestAsk"]),
                    parse_json_as_f64(&parsed_ticker["data"]["bestBid"]),
                ) else {
                    continue;
                };

                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), parsed_buy_price),
                    sell_price: (cfg.name.to_owned(), parsed_sell_price),
                };
                if let Some(pair) =
                    TradingPair::from_str_with_separator(&parsed_ticker["subject"].to_string(), '-')
                {
                    trading_pairs.insert(pair, price);
                }
            }
        }
        _ => {
            return Err(format!(
                "WebSocket interface is not available for {} or STREAM reader not implemented yet",
                exchange.config().name
            )
            .into());
        }
    };
    if trading_pairs.is_empty() {
        return Err("Error: Trading Pairs is empty".into());
    }

    Ok(trading_pairs)
}

fn read_ws_state(
    state: &String,
    exchange: &Exchange,
) -> Result<Tickers, Box<dyn std::error::Error>> {
    let mut trading_pairs: Tickers = HashMap::new();

    match exchange {
        Exchange::Binance(cfg) => {
            let parsed_tickers = json::parse(&state).or(Err(format!(
                "Cannot parse ticker response from {} to JSON",
                cfg.name
            )))?;
            //     "q": "18",              // Total traded quote asset volume

            for ticker in parsed_tickers.members() {
                let symbol = &ticker["s"];
                let parsed_price = parse_json_as_f64(&ticker["c"])?;
                let price = TickerPrice {
                    buy_price: (cfg.name.to_owned(), parsed_price),
                    sell_price: (cfg.name.to_owned(), parsed_price),
                };
                if let Some(pair) = TradingPair::from_str(&symbol.to_string()) {
                    trading_pairs.insert(pair, price);
                }
            }
        }
        Exchange::Mexc(cfg) => {
            for ticker_info in state.split('|') {
                if let Some((symbol, price_str)) = ticker_info.split_once(':') {
                    let Ok(parsed_price) = price_str.parse::<f64>() else {
                        continue;
                    };
                    let price = TickerPrice {
                        buy_price: (cfg.name.to_owned(), parsed_price),
                        sell_price: (cfg.name.to_owned(), parsed_price),
                    };
                    if let Some(pair) = TradingPair::from_str(symbol) {
                        trading_pairs.insert(pair, price);
                    }
                }
            }
        }
        _ => {
            return Err(format!(
                "WebSocket interface is not available for {} or STRING reader not implemented yet",
                exchange.config().name
            )
            .into());
        }
    }

    // if trading_pairs.is_empty() {
    //     return Err("Error: Trading Pairs is empty".into());
    // }

    Ok(trading_pairs)
}
async fn establish_ws_connection_and_sub(
    client: &WebSocketClient,
    exchange: &Exchange,
) -> Result<bool, Box<dyn Error>> {
    let client_clone = client.clone();

    match exchange {
        Exchange::Binance(_) => {
            tokio::spawn(async move {
                client_clone
                    .run_with_reconnect(Some(
                        r#"{"method":"SUBSCRIBE","params":["!ticker@arr"],"id":1}"#,
                    ))
                    .await;
            });
        }
        Exchange::Kucoin(_) => {
            tokio::spawn(async move {
                client_clone
                    .run_with_reconnect(Some(
                        r#"{
                                "id": 123,
                                "type": "subscribe",
                                "topic": "/market/ticker:all",
                                "response": true
                                }"#,
                    ))
                    .await;
            });
        }
        Exchange::Mexc(_) => {
            tokio::spawn(async move {
                client_clone
                    .run_with_reconnect(Some(
                        r#"{
                                "method": "SUBSCRIPTION",
                                "params": ["spot@public.miniTickers.v3.api.pb@UTC+0"]}"#,
                    ))
                    .await;
            });
        }
        Exchange::Huobi(_) => {
            tokio::spawn(async move {
                client_clone
                    .run_with_reconnect(Some(
                        r#"{
                            "sub": "market.tickers"
                        }"#,
                    ))
                    .await;
            });
        }
        _ => {
            return Err(format!(
                "WebSocket interface is not available for {}",
                exchange.config().name
            )
            .into());
        }
    }

    Ok(true)
}
