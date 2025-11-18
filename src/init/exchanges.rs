use crate::config;
use crate::core::exchanges::{Binance, Bybit};
use crate::core::traits::ExchangeStatic;
use crate::core::types::Exchanges;
// use crate::core::exchanges::gate::Gate;
// use crate::core::exchanges::{Bitget, Kucoin, Lbank};

pub async fn get_exchanges() -> Exchanges {
    let mut exchanges = Exchanges::new();

    let Ok(binance) = Binance::new(
        config::binance::NAME,
        config::binance::API_KEY,
        config::binance::SECRET_KEY,
        config::binance::BASE_URL,
        config::binance::WEBSOCKET_URL,
        config::binance::EXCLUDED_PAIRS,
    ) else {
        panic!()
    };
    exchanges.insert(binance.name().to_string(), Box::new(binance));

    let Ok(bybit) = Bybit::new(
        config::bybit::NAME,
        config::bybit::API_KEY,
        config::bybit::SECRET_KEY,
        config::bybit::BASE_URL,
        config::bybit::EXCLUDED_PAIRS,
    ) else {
        panic!();
    };
    exchanges.insert(bybit.name().to_string(), Box::new(bybit));

    // let mexc: Arc<dyn ExchangeAPI> = Arc::new(Mexc::new(
    //     config::mexc::NAME,
    //     config::mexc::API_KEY,
    //     config::mexc::SECRET_KEY,
    //     config::mexc::BASE_URL,
    //     config::mexc::EXCLUDED_PAIRS,
    // ));

    // let huobi: Arc<dyn ExchangeAPI> = Arc::new(Huobi::new(
    //     config::huobi::NAME,
    //     config::huobi::API_KEY,
    //     config::huobi::SECRET_KEY,
    //     config::huobi::BASE_URL,
    //     config::huobi::EXCLUDED_PAIRS,
    // ));

    // let bitget: Arc<dyn ExchangeAPI> = Arc::new(Bitget::new(
    //     config::bitget::NAME,
    //     config::bitget::API_KEY,
    //     config::bitget::SECRET_KEY,
    //     config::bitget::BASE_URL,
    //     config::bitget::EXCLUDED_PAIRS,
    // ));

    // let lbank: Arc<dyn ExchangeAPI> = Arc::new(Lbank::new(
    //     config::lbank::NAME,
    //     config::lbank::API_KEY,
    //     config::lbank::SECRET_KEY,
    //     config::lbank::BASE_URL,
    //     config::lbank::EXCLUDED_PAIRS,
    // ));
    // let kucoin: Arc<dyn ExchangeAPI> = Arc::new(Kucoin::new(
    //     config::kucoin::NAME,
    //     config::kucoin::API_KEY,
    //     config::kucoin::SECRET_KEY,
    //     config::kucoin::BASE_URL,
    //     config::kucoin::EXCLUDED_PAIRS,
    // ));
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
