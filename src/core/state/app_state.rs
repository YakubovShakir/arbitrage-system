use crate::core::types::{PriceData, TradingPair, exchanges::Exchange};
use dashmap::DashMap;
use std::{collections::HashMap, sync::Arc};

pub type ExchangeName = String;
pub type Exchanges = HashMap<ExchangeName, Exchange>;
pub type Tickers = DashMap<TradingPair, PriceData>;

// Состояние всего приложения
pub struct AppState {
    exchanges: Arc<Exchanges>,
    spot_tickers: Arc<Tickers>,
    futures_tickers: Arc<Tickers>,
}
