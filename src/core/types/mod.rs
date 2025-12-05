pub mod api;
pub mod exchanges;
pub mod network;
pub mod price_data;
pub mod sys_primitive;
pub mod trading_pair;
pub use {
    api::API, network::Network, price_data::PriceData, sys_primitive::*, trading_pair::TradingPair,
};

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
