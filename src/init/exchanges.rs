use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio_tungstenite::connect_async;

use crate::config;
use crate::config::parameters::{
    BINANCE_REQUESTS_PER_SECOND, BITGET_REQUESTS_PER_SECOND, BITMART_REQUESTS_PER_SECOND,
    BYBIT_REQUESTS_PER_SECOND, GATE_REQUESTS_PER_SECOND, HUOBI_REQUESTS_PER_SECOND,
    KUCOIN_REQUESTS_PER_SECOND, MARGIN_INFO_CACHE_TTL_SECS, MEXC_REQUESTS_PER_SECOND,
    NETWORKS_CACHE_TTL_SECS, OKX_REQUESTS_PER_SECOND,
};
use crate::core::net::http::HttpClient;
use crate::core::net::websocket::{BinaryMessageHandler, ConnectionHandler, WebSocketClient};
use crate::core::types::Exchanges;
use crate::core::types::api::CacheData;
use crate::core::types::exchanges::{CachedConfig, Exchange, ExchangeConfig};
use crate::core::utils::json_utils::find_value_from_json_key;

pub fn init_binance() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Binance(ExchangeConfig {
        name: config::binance::NAME.to_owned(),
        api_key: env::var("BINANCE_API_KEY")?.to_owned(),
        secret_key: env::var("BINANCE_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::binance::BASE_URL, BINANCE_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(
            config::binance::BASE_URL,
            BINANCE_REQUESTS_PER_SECOND,
        )?,
        websocket_client: Some(WebSocketClient::new(config::binance::WEBSOCKET_URL)),
        futures_websocket_client: Some(WebSocketClient::new(
            config::binance::FUTURES_WEBSOCKET_URL,
        )),
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_bybit() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Bybit(ExchangeConfig {
        name: config::bybit::NAME.to_owned(),
        api_key: env::var("BYBIT_API_KEY")?.to_owned(),
        secret_key: env::var("BYBIT_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bybit::BASE_URL, BYBIT_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(config::bybit::BASE_URL, BYBIT_REQUESTS_PER_SECOND)?,

        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_mexc() -> Result<Exchange, Box<dyn Error>> {
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
    Ok(Exchange::Mexc(ExchangeConfig {
        name: config::mexc::NAME.to_owned(),
        api_key: env::var("MEXC_API_KEY")?.to_owned(),
        secret_key: env::var("MEXC_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::mexc::BASE_URL, MEXC_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(config::mexc::BASE_URL, MEXC_REQUESTS_PER_SECOND)?,

        websocket_client: Some(
            WebSocketClient::new(config::mexc::WEBSOCKET_URL)
                .with_ping_interval(Duration::from_secs(20), r#"{"method": "PING"}"#.to_string())
                .with_binary_handler(mexc_binary_handler),
        ),
        futures_websocket_client: Some(
            WebSocketClient::new(config::mexc::FUTUTES_WEBSOCKET_URL)
                .with_ping_interval(Duration::from_secs(20), r#"{"method": "ping"}"#.to_string()),
        ),
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_bitget() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Bitget(ExchangeConfig {
        name: config::bitget::NAME.to_owned(),
        api_key: env::var("BITGET_API_KEY")?.to_owned(),
        secret_key: env::var("BITGET_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bitget::BASE_URL, BITGET_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(config::bitget::BASE_URL, BITGET_REQUESTS_PER_SECOND)?,
        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_kucoin() -> Result<Exchange, Box<dyn Error>> {
    let kucoin_http_client = HttpClient::new(config::kucoin::BASE_URL, KUCOIN_REQUESTS_PER_SECOND)?;
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

    let futures_connection_handler: ConnectionHandler = Arc::new({
        let http_client =
            HttpClient::new(config::kucoin::FUTURES_BASE_URL, KUCOIN_REQUESTS_PER_SECOND)?;
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

    Ok(Exchange::Kucoin(ExchangeConfig {
        name: kucoin_name.to_owned(),
        api_key: env::var("KUCOIN_API_KEY")?.to_owned(),
        secret_key: env::var("KUCOIN_SECRET_KEY")?.to_owned(),
        http_client: kucoin_http_client.clone(),
        futures_http_client: kucoin_http_client,
        websocket_client: Some(
            WebSocketClient::new(config::kucoin::BASE_URL)
                .with_ping_interval(
                    Duration::from_secs(20),
                    r#"{"id": "123","type": "ping"}"#.to_string(),
                )
                .with_connection_handler(kucoin_connection_handler) // ← просто передаем замыкание
                .with_streamed_state(),
        ),
        futures_websocket_client: Some(
            WebSocketClient::new(config::kucoin::FUTURES_BASE_URL)
                .with_ping_interval(
                    Duration::from_secs(20),
                    r#"{"id": "124","type": "ping"}"#.to_string(),
                )
                .with_connection_handler(futures_connection_handler) // ← просто передаем замыкание
                .with_streamed_state(),
        ),
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}

pub fn init_huobi() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Huobi(ExchangeConfig {
        name: config::huobi::NAME.to_owned(),
        api_key: env::var("HUOBI_API_KEY")?.to_owned(),
        secret_key: env::var("HUOBI_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::huobi::BASE_URL, HUOBI_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(
            config::huobi::FUTURES_BASE_URL,
            HUOBI_REQUESTS_PER_SECOND,
        )?,
        // websocket_client: Some(
        //     WebSocketClient::new(config::huobi::WEBSOCKET_URL)
        //         .with_binary_handler(huobi_binary_handler),
        // ),
        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_gate() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Gate(ExchangeConfig {
        name: config::gate::NAME.to_owned(),
        api_key: env::var("GATE_API_KEY")?.to_owned(),
        secret_key: env::var("GATE_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::gate::BASE_URL, GATE_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(config::gate::BASE_URL, GATE_REQUESTS_PER_SECOND)?,
        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_bitmart() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Bitmart(ExchangeConfig {
        name: config::bitmart::NAME.to_owned(),
        api_key: env::var("BITMART_API_KEY")?.to_owned(),
        secret_key: env::var("BITMART_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::bitmart::BASE_URL, BITMART_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(
            config::bitmart::FUTURES_BASE_URL,
            BITMART_REQUESTS_PER_SECOND,
        )?,

        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}
pub fn init_okx() -> Result<Exchange, Box<dyn Error>> {
    Ok(Exchange::Okx(ExchangeConfig {
        name: config::okx::NAME.to_owned(),
        api_key: env::var("OKX_API_KEY")?.to_owned(),
        secret_key: env::var("OKX_SECRET_KEY")?.to_owned(),
        http_client: HttpClient::new(config::okx::BASE_URL, OKX_REQUESTS_PER_SECOND)?,
        futures_http_client: HttpClient::new(config::okx::BASE_URL, OKX_REQUESTS_PER_SECOND)?,
        websocket_client: None,
        futures_websocket_client: None,
        cached_data: CachedConfig::new(NETWORKS_CACHE_TTL_SECS, MARGIN_INFO_CACHE_TTL_SECS),
    }))
}

pub fn get_exchanges() -> Result<Exchanges, Box<dyn Error>> {
    let mut exchanges: Exchanges = HashMap::new();

    let binance: Exchange = init_binance()?;
    exchanges.insert(config::binance::NAME.to_owned(), binance);

    let bybit: Exchange = init_bybit()?;
    exchanges.insert(config::bybit::NAME.to_owned(), bybit);

    // let mexc: Exchange = init_mexc()?;
    // exchanges.insert(config::mexc::NAME.to_owned(), mexc);

    let bitget: Exchange = init_bitget()?;
    exchanges.insert(config::bitget::NAME.to_owned(), bitget);

    // let kucoin: Exchange = init_kucoin()?;
    // exchanges.insert(config::kucoin::NAME.to_owned(), kucoin);

    let gate: Exchange = init_gate()?;
    exchanges.insert(config::gate::NAME.to_owned(), gate);

    let huobi: Exchange = init_huobi()?;
    exchanges.insert(config::huobi::NAME.to_owned(), huobi);

    let bitmart: Exchange = init_bitmart()?;
    exchanges.insert(config::bitmart::NAME.to_owned(), bitmart);

    let okx: Exchange = init_okx()?;
    exchanges.insert(config::okx::NAME.to_owned(), okx);

    Ok(exchanges)
}
