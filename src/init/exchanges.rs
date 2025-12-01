use core::panic;

use crate::config;
use crate::core::exchanges::{Binance, Bitget, Bybit, Kucoin, Mexc};
use crate::core::traits::ExchangeStatic;
use crate::core::types::Exchanges;

pub async fn get_exchanges() -> Exchanges {
    let mut exchanges = Exchanges::new();

    let Ok(binance) = Binance::new(
        config::binance::NAME,
        config::binance::API_KEY,
        config::binance::SECRET_KEY,
        config::binance::BASE_URL,
        config::binance::WEBSOCKET_URL,
    ) else {
        panic!()
    };
    exchanges.insert(binance.name().to_string(), Box::new(binance));

    let Ok(bybit) = Bybit::new(
        config::bybit::NAME,
        config::bybit::API_KEY,
        config::bybit::SECRET_KEY,
        config::bybit::BASE_URL,
    ) else {
        panic!();
    };
    exchanges.insert(bybit.name().to_string(), Box::new(bybit));

    let Ok(mexc) = Mexc::new(
        config::mexc::NAME,
        config::mexc::API_KEY,
        config::mexc::SECRET_KEY,
        config::mexc::BASE_URL,
        config::mexc::WEBSOCKET_URL,
    ) else {
        panic!();
    };
    exchanges.insert(mexc.name().to_string(), Box::new(mexc));

    // let huobi: Arc<dyn ExchangeAPI> = Arc::new(Huobi::new(
    //     config::huobi::NAME,
    //     config::huobi::API_KEY,
    //     config::huobi::SECRET_KEY,
    //     config::huobi::BASE_URL,
    //     config::huobi::EXCLUDED_PAIRS,
    // ));

    let Ok(bitget) = Bitget::new(
        config::bitget::NAME,
        config::bitget::API_KEY,
        config::bitget::SECRET_KEY,
        config::bitget::BASE_URL,
        // config::bitget::WEBSOCKET_URL,
    ) else {
        panic!();
    };
    exchanges.insert(bitget.name().to_string(), Box::new(bitget));

    // let lbank: Arc<dyn ExchangeAPI> = Arc::new(Lbank::new(
    //     config::lbank::NAME,
    //     config::lbank::API_KEY,
    //     config::lbank::SECRET_KEY,
    //     config::lbank::BASE_URL,
    //     config::lbank::EXCLUDED_PAIRS,
    // ));
    let Ok(kucoin) = Kucoin::new(
        config::kucoin::NAME,
        config::kucoin::API_KEY,
        config::kucoin::SECRET_KEY,
        config::kucoin::BASE_URL,
        config::kucoin::BASE_URL,
    ) else {
        panic!()
    };
    exchanges.insert(kucoin.name().to_string(), Box::new(kucoin));

    // let gate: Arc<dyn ExchangeAPI> = Arc::new(Gate::new(
    //     config::gate::NAME,
    //     config::gate::API_KEY,
    //     config::gate::SECRET_KEY,
    //     config::gate::BASE_URL,
    //     config::gate::EXCLUDED_PAIRS,
    // ));
    // vec![binance, bybit, mexc, huobi, bitget, lbank, kucoin, gate]
    exchanges
}
