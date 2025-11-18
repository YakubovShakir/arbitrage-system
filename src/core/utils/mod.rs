use std::time::{SystemTime, UNIX_EPOCH};

use data_encoding::BASE64;
use hmac::Mac;
use json::JsonValue;

use crate::core::types::{Glass, HmacSha256, Price, Quantity};

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
