pub const USDT_LIMIT: f64 = 1000.0;
pub const PAIRS_PER_THREAD: usize = 15;
pub const EXCHANGES_PER_TICKER_THREAD: usize = 4;
pub const QUOTE_LIST: &[&'static str] = &["USDT"];
pub const REQUIRED_SPREAD_PERCENT: f64 = 0.4;
pub const HTTP_MAX_POOL_IDLE_PER_HOST: usize = 20;
pub const HTTP_TIMEOUT_SECS: u64 = 5;
pub const HTTP_RETRY_AFTER_MILLIS: u64 = 2000;
