use std::time::Duration;

use json::JsonValue;
use tokio::{sync::RwLock, time::Instant};

use super::exchanges::Exchange;

pub struct CacheData {
    value: RwLock<JsonValue>,
    timestamp: RwLock<Instant>,
    ttl: Duration,
}

impl CacheData {
    pub fn new(value: JsonValue, ttl: Duration) -> Self {
        Self {
            value: RwLock::new(value),
            timestamp: RwLock::new(Instant::now()),
            ttl: ttl,
        }
    }
    pub async fn get(&self) -> Option<JsonValue> {
        // ВСЕГДА сначала timestamp, потом value (как в set)
        let timestamp_guard = self.timestamp.read().await;
        let value_guard = self.value.read().await;

        if !value_guard.is_null() && timestamp_guard.elapsed() <= self.ttl {
            Some(value_guard.clone())
        } else {
            None
        }
    }

    pub async fn set(&self, value: JsonValue) {
        let mut timestamp_quard = self.timestamp.write().await;
        let mut value_quard = self.value.write().await;
        *value_quard = value;
        *timestamp_quard = Instant::now();
    }
}

pub enum API {
    GetTickers,
    GetNetworks,
    GetMarginInfo,
    GetOrderBook,
}

impl API {
    pub fn endpoint(&self, exchange: &Exchange) -> Option<&str> {
        match self {
            API::GetTickers => match exchange {
                Exchange::Binance(_) => None,
                Exchange::Bybit(_) => Some("/v5/market/tickers"),
                Exchange::Bitget(_) => Some("/api/v2/spot/market/tickers"),
                Exchange::Gate(_) => Some("/api/v4/spot/tickers"),
                Exchange::Kucoin(_) => None,
                Exchange::Mexc(_) => None,
                Exchange::Huobi(_) => Some("/market/tickers"),
                Exchange::Bitmart(_) => Some("/spot/quotation/v3/tickers"),
                Exchange::Okx(_) => Some("/api/v5/market/tickers"),
            },
            API::GetMarginInfo => match exchange {
                Exchange::Binance(_) => Some("/sapi/v1/margin/allAssets"),
                Exchange::Bybit(_) => Some("/v5/spot-margin-trade/data"),
                Exchange::Bitget(_) => Some("/api/v2/margin/currencies"),
                Exchange::Gate(_) => Some("/api/v4/margin/uni/currency_pairs"),
                Exchange::Kucoin(_) => Some("/api/v3/currencies/"),
                Exchange::Mexc(_) => None,
                Exchange::Huobi(_) => Some("/v1/margin/loan-info"),
                Exchange::Bitmart(_) => None,
                Exchange::Okx(_) => None,
            },
            API::GetNetworks => match exchange {
                Exchange::Binance(_) => Some("/sapi/v1/capital/config/getall"),
                Exchange::Bybit(_) => Some("/v5/asset/coin/query-info"),
                Exchange::Bitget(_) => Some("/api/v2/spot/public/coins"),
                Exchange::Gate(_) => Some("/api/v4/wallet/currency_chains"),
                Exchange::Kucoin(_) => Some("/api/v3/currencies"),
                Exchange::Mexc(_) => Some("/api/v3/capital/config/getall"),
                Exchange::Huobi(_) => Some("/v2/reference/currencies"),
                Exchange::Bitmart(_) => Some("/account/v1/currencies"),
                Exchange::Okx(_) => Some("/api/v5/asset/currencies"),
            },
            API::GetOrderBook => match exchange {
                Exchange::Binance(_) => Some("/api/v3/depth"),
                Exchange::Bybit(_) => Some("/v5/market/orderbook"),
                Exchange::Bitget(_) => Some("/api/v2/spot/market/orderbook"),
                Exchange::Gate(_) => Some("/api/v4/spot/order_book"),
                Exchange::Kucoin(_) => Some("/api/v3/market/orderbook/level2"),
                Exchange::Mexc(_) => Some("/api/v3/depth"),
                Exchange::Huobi(_) => Some("/market/depth"),
                Exchange::Bitmart(_) => Some("/spot/quotation/v3/books"),
                Exchange::Okx(_) => Some("/api/v5/market/books"),
            },
        }
    }
}
