pub mod exchanges;
pub mod parameters;

pub use parameters::PAIRS_PER_THREAD;
pub use parameters::USDT_LIMIT;

pub use exchanges::{binance, bitget, bybit, gate, huobi, kucoin, lbank, mexc};
