use hmac::Hmac;
use sha2::Sha256;

pub mod trading_pair;

pub type Price = f64;

pub type Quantity = f64;

pub type OrderBookLevel = (Price, Quantity);

pub type Glass = Vec<OrderBookLevel>;

pub type Asks = Glass;

pub type Bids = Glass;

pub type OrderBook = (Asks, Bids);

pub type Key = String;

pub type Value = String;

pub type KeyValue = (Key, Value);

pub type HmacSha256 = Hmac<Sha256>;
