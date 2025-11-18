use hmac::Hmac;
use sha2::Sha256;
use std::collections::HashMap;

use crate::core::{
    traits::Exchange,
    types::trading_pair::{PriceData, TradingPair},
};

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

pub type ExchangeName = String;

pub type Exchanges = HashMap<ExchangeName, Box<dyn Exchange>>;

pub type TradingPairs = HashMap<TradingPair, PriceData>;

pub enum RestEndPoint {
    // Получить информацию об адресе депозита по сети
    GetDepositAddressInfo {
        coin: String,
        network: String,
        timestamp: String,
    },
    // Получить информацию о доступных сетях
    GetTransferInfo {
        asset: String,
        timestamp: String,
    },
    // Получить информацию о балансе
    GetAssetBalance {
        asset: String,
        timestamp: String,
    },
    // Получить информацию о возможности взять в долг
    GetMarginStatus {
        asset: String,
    },
    // Метод отправки монет по сети
    WithdrawAsset {
        coin: String,
        address: String,
        network: String,
        amount: String,
        timestamp: String,
    },
    // Внутренний перевод между разными кошельками бирж
    TransferAsset {
        direction: String,
        asset: String,
        amount: String,
        timestamp: String,
    },
    // Взять в долг средства
    BorrowAsset {
        asset: String,
        amount: String,
        timestamp: String,
    },
    // Погасить долг
    RepayAsset {
        asset: String,
        amount: String,
        timestamp: String,
    },
}
