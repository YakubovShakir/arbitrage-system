use chrono::{DateTime, Utc};
use flate2::read::GzDecoder;
use log::{debug, error};
use std::{
    io::Read,
    mem,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

pub mod crypto;
pub mod json_utils;

use crate::{
    config::parameters::{DEBUG_CODE, RESET_CODE, WARNING_CODE},
    core::{
        traits::exchange_service::NetworkService,
        types::{
            ExchangeName, Glass, TradingPair,
            exchanges::Exchange,
            network::{DepositNetwork, WithdrawNetwork},
        },
    },
};

// pub fn find_best_network_by_fee()

pub fn find_intersection_from_networks(
    withdraw_networks: &[WithdrawNetwork],
    deposit_networks: &[DepositNetwork],
) -> Option<Vec<(WithdrawNetwork, DepositNetwork)>> {
    let mut intersection: Vec<(WithdrawNetwork, DepositNetwork)> = Vec::new();

    for w_network in withdraw_networks {
        // ← из withdraw_networks
        for d_network in deposit_networks {
            if &w_network.base.network_type == &d_network.base.network_type {
                intersection.push((w_network.clone(), d_network.clone())); // ← клонируем w_network
                break;
            }
        }
    }

    if intersection.is_empty() {
        None
    } else {
        Some(intersection)
    }
}
pub async fn verify_arbitrage_conditions_and_get_networks(
    buy_exchange: &Exchange,
    sell_exchange: &Exchange,
    trading_pair_name: &TradingPair,
) -> Result<Vec<(WithdrawNetwork, DepositNetwork)>, ExchangeName> {
    let buy_exchange_name = &buy_exchange.config().name;
    let sell_exchange_name = &sell_exchange.config().name;
    // let is_margin_available = match sell_exchange.borrowable(&trading_pair_name).await {
    //     Ok(result) => result,
    //     Err(e) => {
    //         println!(
    //             "{WARNING_CODE}[WARNING] {}/{} verification failed due to can't get borrow status from {}. ERROR: {}{RESET_CODE}",
    //             trading_pair_name.base, trading_pair_name.quote, sell_exchange_name, e
    //         );
    //         // return Err(sell_exchange_name.to_string());
    //         return Ok(Vec::new());
    //     }
    // };

    // if !is_margin_available {
    //     println!(
    //         "{DEBUG_CODE}[DEBUG] {}/{} verify - failed. REASON: {} not borrowable{RESET_CODE}",
    //         trading_pair_name.base, trading_pair_name.quote, sell_exchange_name
    //     );
    //     return Err(sell_exchange_name.to_string());
    // }

    let buy_networks_start = Instant::now();
    let withdraw_networks = match buy_exchange.networks(&trading_pair_name.base).await {
        Ok(networks) => networks.0,
        Err(e) => {
            error!(
                "{}/{} verify returns empty networks. REASON: Can't get buy {} networks. ERROR: {}",
                trading_pair_name.base, trading_pair_name.quote, buy_exchange_name, e
            );
            return Ok(Vec::new());
        }
    };
    let buy_networks_elapsed = buy_networks_start.elapsed().as_nanos();

    if withdraw_networks.len() == 0 {
        debug!(target: "debug_module",
            "{}/{} verify - failed. REASON: Empty buy networks {}",
            trading_pair_name.base, trading_pair_name.quote, buy_exchange_name
        );
        return Err(buy_exchange_name.to_string());
    }

    let sell_networks_start = Instant::now();
    let deposit_networks = match sell_exchange.networks(&trading_pair_name.base).await {
        Ok(networks) => networks.1,
        Err(e) => {
            error!(
                "{}/{} verify returns empty networks. REASON: Can't get sell {} networks. ERROR: {}",
                trading_pair_name.base, trading_pair_name.quote, sell_exchange_name, e
            );
            return Ok(Vec::new());
        }
    };

    let sell_networks_elapsed = sell_networks_start.elapsed().as_nanos();

    if deposit_networks.len() == 0 {
        debug!(target: "debug_module", "{}/{} verify - failed. REASON: Empty sell networks {}",  trading_pair_name.base, trading_pair_name.quote, sell_exchange_name );
        return Err(sell_exchange_name.to_string());
    }

    let Some(networks) = find_intersection_from_networks(&withdraw_networks, &deposit_networks)
    else {
        // println!(
        //     "No network intersection beetween {}:{:#?} and {}:{:#?} at {}/{}",
        //     buy_exchange.config().name,
        //     buy_networks,
        //     sell_exchange.config().name,
        //     sell_networks,
        //     trading_pair_name.base,
        //     trading_pair_name.quote,
        // );

        return Ok(Vec::new());
    };
    debug!(target: "debug_module", "buy_net_el: ({},{}); sell_net_el: ({},{})", buy_exchange_name, format_duration(buy_networks_elapsed), sell_exchange_name, format_duration(sell_networks_elapsed));

    Ok(networks)
}

pub fn comput_spread_percent(buy_price: &f64, sell_price: &f64) -> f64 {
    return (*sell_price / *buy_price - 1.0) * 100.0;
}

pub fn calculate_price_by_glass(quote_limit: &f64, glass: &Glass) -> Option<f64> {
    let mut total_quantity: f64 = 0.0;
    let mut quote_volume: f64;
    let mut quote_volume_left: f64 = *quote_limit;

    for level in glass {
        if level.0 <= 0.0 || level.1 <= 0.0 {
            continue;
        }

        quote_volume = level.0 * level.1;
        if quote_volume_left - quote_volume <= 0.0 {
            total_quantity += quote_volume_left / level.0;
            quote_volume_left = 0.0;
            break;
        }

        total_quantity += level.1;
        quote_volume_left -= quote_volume;
    }
    if quote_volume_left > 0.0 || total_quantity <= 0.0 {
        return None;
    };
    Some(quote_limit / total_quantity)
}

pub fn get_timestamp_iso_8601() -> Result<String, Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    Ok(now.format("%Y-%m-%dT%H:%M:%S").to_string())
}
pub fn get_timestamp_iso_8601_with_ms() -> Result<String, Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    Ok(now.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string())
}
pub fn get_current_timestamp() -> Result<String, Box<dyn std::error::Error>> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string())
}

pub fn get_current_timestamp_secs() -> Result<String, Box<dyn std::error::Error>> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string())
}

pub fn format_duration(nanos: u128) -> String {
    if nanos > 1_000_000_000 {
        // > 1 секунда
        format!("{:.3} s", nanos as f64 / 1_000_000_000.0)
    } else if nanos > 1_000_000 {
        // > 1 миллисекунда
        format!("{:.3} ms", nanos as f64 / 1_000_000.0)
    } else if nanos > 1_000 {
        // > 1 микросекунда
        format!("{:.3} µs", nanos as f64 / 1_000.0)
    } else {
        // наносекунды
        format!("{} ns", nanos)
    }
}

pub fn decompress_gzip(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;
    Ok(decompressed)
}
