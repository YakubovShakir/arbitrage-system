use std::time::{SystemTime, UNIX_EPOCH};

use data_encoding::BASE64;
use hmac::Mac;
use json::JsonValue;

use crate::core::{
    traits::Exchange,
    types::{
        Glass, HmacSha256, Price, Quantity,
        structs::{Network, TradingPair},
    },
};

pub fn find_value_from_json_key(
    json: &JsonValue,
    key_order: &[&str],
) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut value = json;
    for key in key_order {
        if value.has_key(key) {
            value = &value[*key];
        } else {
            return Err(format!(
                "Could not find Key<{:?}> because Key<{}> was not found in path",
                key_order.last(),
                key,
            )
            .into());
        }
    }
    Ok(value.clone())
}

// pub fn find_best_network_by_fee()

pub fn find_intersection_from_networks(
    withdraw_networks: &Vec<Network>,
    deposit_networks: &Vec<Network>,
) -> Option<Vec<Network>> {
    let mut intersection: Vec<Network> = Vec::new();

    for w_network in withdraw_networks {
        for d_network in deposit_networks {
            if w_network.name.to_uppercase() == d_network.name.to_uppercase() {
                intersection.push(w_network.clone());
                break;
            }

            if let (Some(addr1), Some(addr2)) =
                (&w_network.contract_address, &d_network.contract_address)
            {
                if addr1 == addr2 {
                    intersection.push(w_network.clone());
                    break;
                }
            }
            if w_network.full_name.to_uppercase() == d_network.full_name.to_uppercase() {
                intersection.push(w_network.clone());
                break;
            }
        }
    }
    if intersection.len() == 0 {
        return None;
    };
    Some(intersection)
}

pub async fn verify_arbitrage_conditions_and_get_networks(
    buy_exchange: &Box<dyn Exchange>,
    sell_exchange: &Box<dyn Exchange>,
    trading_pair_name: &TradingPair,
) -> Option<Vec<Network>> {
    let is_margin_available = match sell_exchange
        .is_margin_available(&trading_pair_name.base)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            // println!("Ошибка вызова is_margin_available {}", e);
            return None;
        }
    };

    if !is_margin_available {
        return None;
    }
    let buy_networks = match buy_exchange.fetch_networks(&trading_pair_name.base).await {
        Ok(networks) => networks,
        Err(e) => {
            println!(
                "Не удалось получить сети с биржи покупки, {} - {}",
                buy_exchange.name(),
                e
            );
            return None;
        }
    };

    let sell_networks = match sell_exchange.fetch_networks(&trading_pair_name.base).await {
        Ok(networks) => networks,
        Err(e) => {
            println!(
                "Не удалось получить сети с биржи продажи, {} - {}",
                sell_exchange.name(),
                e
            );
            return None;
        }
    };

    let Some(networks) = find_intersection_from_networks(&buy_networks, &sell_networks) else {
        println!(
            "No network intersection beetween {} and {} at {}/{}",
            buy_exchange.name(),
            sell_exchange.name(),
            trading_pair_name.base,
            trading_pair_name.quote,
        );
        return None;
    };

    Some(networks)
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

pub fn parse_json_value_as_f64(value: &JsonValue) -> Result<f64, Box<dyn std::error::Error>> {
    value
        .to_string()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse '{}' as f64: {}", value, e).into())
}

// В нулевом индексе пары должна быть цена, а в первом количество
pub fn parse_string_typed_glass(
    glass: &JsonValue,
) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for order_level in glass.members() {
        let price: Price = order_level[0]
            .as_str()
            .ok_or("Cannot parse price json as str")?
            .parse()?;
        let quantity: Quantity = order_level[1]
            .as_str()
            .ok_or("Cannot parse quantity json as str")?
            .parse()?;
        result.push((price, quantity));
    }
    Ok(result)
}

// В нулевом индексе пары должна быть цена, а в первом количество
pub fn parse_number_typed_glass(
    glass: &JsonValue,
) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for order_level in glass.members() {
        let price: Price = order_level[0]
            .as_f64()
            .ok_or("Cannot parse price json as number")?;

        let quantity: Quantity = order_level[1]
            .as_f64()
            .ok_or("Cannot parse quantity json as str")?;
        result.push((price, quantity));
    }
    Ok(result)
}

pub fn get_current_timestamp() -> Result<String, Box<dyn std::error::Error>> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string())
}

pub fn encrypt_hmac_sha256(
    secret_key: &String,
    signature: &String,
) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    Ok(HmacSha256::new_from_slice(secret_key.as_bytes())?
        .chain_update(signature.as_bytes())
        .finalize()
        .into_bytes()
        .into())
}

pub fn hex_encode<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
}

pub fn base64_encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

// pub fn print_dashboard(trading_pairs: Vec<Arc<TradingPair>>, exchanges: Vec<Arc<dyn Exchange>>) {
//     println!(
//         "\n{}\n| {:<4} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} |",
//         "-".repeat(86),
//         "No.",
//         "TRADING PAIR",
//         "BUY EXCHANGE",
//         "BUY PRICE",
//         "SELL EXCHANGE",
//         "SELL PRICE"
//     );
//     let mut dashboard = format!("\n{}\n", "-".repeat(86));
//     let mut no = 0;
//     for pair in trading_pairs {
//         no += 1;
//         let pair_buy = match pair.min_buy_price() {
//             Some((exchange_index, price)) => {
//                 let exchange_name = match exchanges.get(exchange_index) {
//                     Some(exchange) => exchange.name(),
//                     None => "None",
//                 };
//                 (exchange_name, price)
//             }
//             None => ("None", 0.0),
//         };

//         let pair_sell = match pair.max_sell_price() {
//             Some((exchange_index, price)) => {
//                 let exchange_name = match exchanges.get(exchange_index) {
//                     Some(exchange) => exchange.name(),
//                     None => "None",
//                 };
//                 (exchange_name, price)
//             }
//             None => ("None", 0.0),
//         };

//         dashboard += &format!(
//             "\n| {:<4} | {:<6}/{:<7} | {:<14} | {:<14.8} | {:<14} | {:<14.8} |",
//             no,
//             pair.base(),
//             pair.quote(),
//             pair_buy.0,
//             pair_buy.1,
//             pair_sell.0,
//             pair_sell.1
//         );
//     }

//     println!("{}", dashboard);
// }
