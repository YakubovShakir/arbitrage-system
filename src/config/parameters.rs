pub const PAIRS_PER_THREAD: usize = 15;
pub const EXCHANGES_PER_TICKER_THREAD: usize = 4;

pub const QUOTE_LIST: &[(&str, f64)] = &[
    ("USDT", 900.0),
    ("USDC", 900.0),
    ("DAI", 900.0),
    ("TUSD", 900.0),
    ("BTC", 0.01),
    ("ETH", 0.3),
];
pub const QUOTE_RATE: f64 = 1.5;
pub const REQUIRED_TICKER_SPREAD_PERCENT: f64 = 0.25;
pub const REQUIRED_ORDERBOOK_SPREAD_PERCENT: f64 = 0.25;
pub const NETWORKS_CACHE_TTL_SECS: u64 = 1200;
pub const MARGIN_INFO_CACHE_TTL_SECS: u64 = 1200;

pub const REQUESTS_CHUNK_SIZE: usize = 20;

pub const HTTP_MAX_POOL_IDLE_PER_HOST: usize = 20;
pub const HTTP_RETRY_AFTER_MILLIS: u64 = 300;

pub const GLOBAL_HTTP_TIMEOUT_SECS: u64 = 5;

// BINANCE REQUEST TIMEOUTS
pub const BINANCE_NETWORKS_HTTP_TIMEOUT_SECONDS: u64 = 10;

// BITGET REQUEST TIMEOUTS
pub const BITGET_MARGIN_INFO_HTTP_TIMEOUT_SECONDS: u64 = 10;
pub const BITGET_NETWORKS_HTTP_TIMEOUT_SECONDS: u64 = 10;

// GATE.IO REQUEST TIMEOUTS
pub const GATE_MARGIN_INFO_HTTP_TIMEOUT_SECONDS: u64 = 10;
pub const GATE_TICKERS_HTTP_TIMEOUT_SECONDS: u64 = 10;
pub const GATE_NETWORKS_HTTP_TIMEOUT_SECONDS: u64 = 5;
// MEXC REQUEST TIMEOUTS
pub const MEXC_NETWORKS_HTTP_TIMEOUT_SECONDS: u64 = 10;

// HUOBI REQUEST TIMEOUTS
pub const HUOBI_TICKERS_HTTP_TIMEOUT_SECONDS: u64 = 30;
pub const HUOBI_MARGIN_INFO_HTTP_TIMEOUT_SECONDS: u64 = 10;

// Rate Limits
pub const BINANCE_REQUESTS_PER_SECOND: usize = 10;
pub const BYBIT_REQUESTS_PER_SECOND: usize = 5;
pub const BITGET_REQUESTS_PER_SECOND: usize = 5;
pub const BITMART_REQUESTS_PER_SECOND: usize = 5;
pub const GATE_REQUESTS_PER_SECOND: usize = 450;
pub const HUOBI_REQUESTS_PER_SECOND: usize = 100;
pub const MEXC_REQUESTS_PER_SECOND: usize = 15;
pub const OKX_REQUESTS_PER_SECOND: usize = 20;
pub const KUCOIN_REQUESTS_PER_SECOND: usize = 15;

// pub const BINANCE_REQUESTS_PER_SECOND: usize = 0;
// pub const BYBIT_REQUESTS_PER_SECOND: usize = 0;
// pub const BITGET_REQUESTS_PER_SECOND: usize = 0;
// pub const BITMART_REQUESTS_PER_SECOND: usize = 0;
// pub const GATE_REQUESTS_PER_SECOND: usize = 0;
// pub const HUOBI_REQUESTS_PER_SECOND: usize = 0;
// pub const MEXC_REQUESTS_PER_SECOND: usize = 0;
// pub const OKX_REQUESTS_PER_SECOND: usize = 0;
// pub const KUCOIN_REQUESTS_PER_SECOND: usize = 0;
pub const SUCCESS_CODE: &str = "\x1b[38;5;48m"; // #00ff87 - яркий мятный ✅
pub const ERROR_CODE: &str = "\x1b[38;5;203m"; // #ff5f5f - яркий алый ❌  
pub const WARNING_CODE: &str = "\x1b[38;5;221m"; // #ffd75f - тёплый жёлтый ⚠️
pub const INFO_CODE: &str = "\x1b[38;5;45m"; // #00d7ff - аквамарин 💎
pub const DEBUG_CODE: &str = "\x1b[38;5;247m"; // #9e9e9e - средний серый 🔍
pub const RESET_CODE: &str = "\x1b[0m";
