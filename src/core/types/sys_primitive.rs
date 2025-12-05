use hmac::Hmac;
use sha2::{Sha256, Sha512};
use std::collections::HashMap;

use super::{PriceData, TradingPair};
use crate::core::traits::Exchange;

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
pub type HmacSha512 = Hmac<Sha512>;
pub type ExchangeName = String;
pub type Exchanges = HashMap<ExchangeName, Box<dyn Exchange>>;

pub type TradingPairs = HashMap<TradingPair, PriceData>;
