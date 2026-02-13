use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio_tungstenite::connect_async;

use crate::config;
use crate::config::parameters::{
    MARGIN_INFO_CACHE_TTL_SECS, NETWORKS_CACHE_TTL_SECS, RESET_CODE, SUCCESS_CODE,
};
use crate::core::net::http::HttpClient;
use crate::core::net::websocket::{BinaryMessageHandler, ConnectionHandler, WebSocketClient};
use crate::core::types::Exchanges;
use crate::core::types::api::CacheData;
use crate::core::types::exchanges::{CachedConfig, Exchange, ExchangeConfig};
use crate::core::types::rate_limiter::RateLimiter;
use crate::core::utils::json_utils::find_value_from_json_key;

pub fn get_exchanges() -> Result<Exchanges, Box<dyn Error>> {
    let mut exchanges: Exchanges = HashMap::new();

    // Binance
    let binance = Exchange::Binance(ExchangeConfig {
        name: config::binance::NAME.to_owned(),
        api_key: env::var("BINANCE_API_KEY")?.to_owned(),
        secret_key: env::var("BINANCE_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::binance::BASE_URL, 50)?,
        websocket_client: Some(WebSocketClient::new(config::binance::WEBSOCKET_URL)),
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::binance::NAME.to_owned(), binance);

    // Bybit
    let bybit = Exchange::Bybit(ExchangeConfig {
        name: config::bybit::NAME.to_owned(),
        api_key: env::var("BYBIT_API_KEY")?.to_owned(),
        secret_key: env::var("BYBIT_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bybit::BASE_URL, 80)?,
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::bybit::NAME.to_owned(), bybit);

    // Mexc
    // Обработчик для protobuf сообщений MEXC
    let mexc_binary_handler: BinaryMessageHandler = Arc::new(|data: prost::bytes::Bytes| {
        match crate::config::mexc_protocol_buffers::decode_message(&data) {
            Ok(message) => {
                let tickers =
                    crate::config::mexc_protocol_buffers::handle_protobuf_message(message);

                // Преобразуем тикеры в строку для состояния
                if !tickers.is_empty() {
                    let ticker_strings: Vec<String> = tickers
                        .iter()
                        .map(|t| format!("{}:{}", t.symbol, t.price))
                        .collect();
                    Some(ticker_strings.join("|"))
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("Ошибка декодирования protobuf MEXC: {}", e);
                None
            }
        }
    });
    let mexc = Exchange::Mexc(ExchangeConfig {
        name: config::mexc::NAME.to_owned(),
        api_key: env::var("MEXC_API_KEY")?.to_owned(),
        secret_key: env::var("MEXC_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::mexc::BASE_URL, 120)?,
        websocket_client: Some(
            WebSocketClient::new(config::mexc::WEBSOCKET_URL)
                .with_ping_interval(Duration::from_secs(20), r#"{"method": "PING"}"#.to_string())
                .with_binary_handler(mexc_binary_handler),
        ),
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::mexc::NAME.to_owned(), mexc);

    // Bitget
    let bitget = Exchange::Bitget(ExchangeConfig {
        name: config::bitget::NAME.to_owned(),
        api_key: env::var("BITGET_API_KEY")?.to_owned(),
        secret_key: env::var("BITGET_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bitget::BASE_URL, 15)?,
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::bitget::NAME.to_owned(), bitget);

    // Kucoin
    let kucoin_http_client = HttpClient::new(config::kucoin::BASE_URL, 130)?;
    let kucoin_name = config::kucoin::NAME;
    let kucoin_connection_handler: ConnectionHandler = Arc::new({
        let http_client = kucoin_http_client.clone();
        let name = kucoin_name.to_string();

        move || {
            let http_client = http_client.clone();
            let name = name.clone();

            Box::pin(async move {
                // Вся логика подключения
                let response = match http_client
                    .post("/api/v1/bullet-public", None, None, None)
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        return Err(format!("{} error in ConnectionHandler - {}", name, e).into());
                    }
                };

                if response["code"] != "200000" {
                    return Err(format!("API error from {}", name).into());
                }

                let token = match find_value_from_json_key(&response, &["data", "token"]) {
                    Ok(token) => token,
                    Err(e) => {
                        return Err(format!("{} error in ConnectionHandler - {}", name, e).into());
                    }
                };

                let servers =
                    match find_value_from_json_key(&response, &["data", "instanceServers"]) {
                        Ok(servers) => servers,
                        Err(e) => {
                            return Err(
                                format!("{} error in ConnectionHandler - {}", name, e).into()
                            );
                        }
                    };

                let endpoint = match find_value_from_json_key(&servers[0], &["endpoint"]) {
                    Ok(endpoint) => endpoint,
                    Err(e) => {
                        return Err(format!("{} error in ConnectionHandler - {}", name, e).into());
                    }
                };

                let url = format!("{}?token={}", endpoint, token);

                let result = connect_async(&url).await?;
                info!(target: "info_module", "🔗 WebSocket соединение c {} установлено", endpoint);

                Ok(result)
            })
        }
    });
    let kucoin = Exchange::Kucoin(ExchangeConfig {
        name: kucoin_name.to_owned(),
        api_key: env::var("KUCOIN_API_KEY")?.to_owned(),
        secret_key: env::var("KUCOIN_SECRET_KEY")?.to_owned(),
        http_client: kucoin_http_client,
        websocket_client: Some(
            WebSocketClient::new(config::kucoin::BASE_URL)
                .with_ping_interval(
                    Duration::from_secs(20),
                    r#"{"id": "123","type": "ping"}"#.to_string(),
                )
                .with_connection_handler(kucoin_connection_handler) // ← просто передаем замыкание
                .with_streamed_state(),
        ),
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::kucoin::NAME.to_owned(), kucoin);

    // Gate.io
    let gate = Exchange::Gate(ExchangeConfig {
        name: config::gate::NAME.to_owned(),
        api_key: env::var("GATE_API_KEY")?.to_owned(),
        secret_key: env::var("GATE_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::gate::BASE_URL, 900)?,
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::gate::NAME.to_owned(), gate);

    let huobi = Exchange::Huobi(ExchangeConfig {
        name: config::huobi::NAME.to_owned(),
        api_key: env::var("HUOBI_API_KEY")?.to_owned(),
        secret_key: env::var("HUOBI_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::huobi::BASE_URL, 200)?,
        // websocket_client: Some(
        //     WebSocketClient::new(config::huobi::WEBSOCKET_URL)
        //         .with_binary_handler(huobi_binary_handler),
        // ),
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::huobi::NAME.to_owned(), huobi);

    let bitmart = Exchange::Bitmart(ExchangeConfig {
        name: config::bitmart::NAME.to_owned(),
        api_key: env::var("BITMART_API_KEY")?.to_owned(),
        secret_key: env::var("BITMART_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bitmart::BASE_URL, 20)?,
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::bitmart::NAME.to_owned(), bitmart);

    let okx = Exchange::Okx(ExchangeConfig {
        name: config::okx::NAME.to_owned(),
        api_key: env::var("OKX_API_KEY")?.to_owned(),
        secret_key: env::var("OKX_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::okx::BASE_URL, 60)?,
        websocket_client: None,
        cached_data: CachedConfig {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(NETWORKS_CACHE_TTL_SECS),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(MARGIN_INFO_CACHE_TTL_SECS),
            ),
        },
    });
    exchanges.insert(config::okx::NAME.to_owned(), okx);
    Ok(exchanges)
}
