use chrono::{DateTime, Utc};
use ring::digest;
use std::time::{SystemTime, UNIX_EPOCH};

use data_encoding::BASE64;
use hmac::Mac;
use json::JsonValue;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use sha2::{Digest, Sha256};

use crate::core::{
    traits::exchange_service::{MarginInfoService, NetworkService},
    types::{Glass, HmacSha256, HmacSha512, Network, TradingPair, exchanges::Exchange},
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
                if addr1.to_uppercase() == addr2.to_uppercase() {
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
    buy_exchange: &Exchange,
    sell_exchange: &Exchange,
    trading_pair_name: &TradingPair,
) -> Option<Vec<Network>> {
    let is_margin_available = match sell_exchange.borrowable(&trading_pair_name).await {
        Ok(result) => result,
        Err(_) => {
            // println!("Ошибка вызова is_margin_available {}", e);
            return None;
        }
    };

    if !is_margin_available {
        return None;
    }
    let buy_networks = match buy_exchange.networks(&trading_pair_name.base).await {
        Ok(networks) => networks,
        Err(e) => {
            println!(
                "Не удалось получить сети с биржи покупки, {} - {}",
                buy_exchange.config().name,
                e
            );
            return None;
        }
    };

    let sell_networks = match sell_exchange.networks(&trading_pair_name.base).await {
        Ok(networks) => networks,
        Err(e) => {
            println!(
                "Не удалось получить сети с биржи продажи, {} - {}",
                sell_exchange.config().name,
                e
            );
            return None;
        }
    };

    let Some(networks) = find_intersection_from_networks(&buy_networks, &sell_networks) else {
        // println!(
        //     "No network intersection beetween {}:{:#?} and {}:{:#?} at {}/{}",
        //     buy_exchange.name(),
        //     buy_networks,
        //     sell_exchange.name(),
        //     sell_networks,
        //     trading_pair_name.base,
        //     trading_pair_name.quote,
        // );
        // println!(
        //     "No network intersection beetween {} and {} at {}/{}",
        //     buy_exchange.name(),
        //     sell_exchange.name(),
        //     trading_pair_name.base,
        //     trading_pair_name.quote,
        // );
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

pub fn parse_json_as_str(value: &JsonValue) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = value
        .as_str()
        .ok_or("Cannot parse value as str")?
        .to_string();
    if parsed.is_empty() || parsed.to_lowercase() == "null" || parsed.to_lowercase() == "none" {
        return Err(format!("Cannot parse value as str: value is empty or null or none").into());
    }
    Ok(parsed)
}

pub fn parse_json_as_f64(value: &JsonValue) -> Result<f64, Box<dyn std::error::Error>> {
    value
        .to_string()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse '{}' as f64: {}", value, e).into())
}
pub fn parse_json_as_bool(value: &JsonValue) -> Result<bool, Box<dyn std::error::Error>> {
    // Сначала пытаемся как булево значение
    if let Some(bool_val) = value.as_bool() {
        return Ok(bool_val);
    }
    if let Some(int_val) = value.as_u8() {
        match int_val {
            0 => return Ok(false),
            1 => return Ok(true),
            _ => {}
        }
    }
    // Если не получилось, пытаемся как строку
    if let Some(str_val) = value.as_str() {
        match str_val.to_lowercase().as_str() {
            "true" => return Ok(true),
            "false" => return Ok(false),
            _ => {}
        }
    }

    Err("Could not parse value as boolean".into())
}

pub fn get_timestamp_iso_8601() -> Result<String, Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    Ok(now.format("%Y-%m-%dT%H:%M:%S").to_string())
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

pub fn sha512_with_ring_return_hex(input: &str) -> String {
    let digest = digest::digest(&digest::SHA512, input.as_bytes());
    hex::encode(digest.as_ref())
}
pub fn encrypt_hmac_sha256(
    secret_key: &str,
    signature: &str,
) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    Ok(HmacSha256::new_from_slice(secret_key.as_bytes())?
        .chain_update(signature.as_bytes())
        .finalize()
        .into_bytes()
        .into())
}

pub fn encrypt_hmac_sha512(
    secret_key: &str,
    signature: &str,
) -> Result<[u8; 64], Box<dyn std::error::Error>> {
    Ok(HmacSha512::new_from_slice(secret_key.as_bytes())?
        .chain_update(signature.as_bytes())
        .finalize()
        .into_bytes()
        .into())
}

pub fn hex_encode<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
}

pub fn sign_rsa_sha256(
    private_key_str: &str,
    message: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use base64::{Engine as _, engine::general_purpose};

    let der_bytes = general_purpose::STANDARD.decode(private_key_str)?;
    let private_key = PKey::private_key_from_der(&der_bytes)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;

    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(message.as_bytes());
    let sha256_hash = sha256_hasher.finalize();

    signer.update(&sha256_hash)?;
    Ok(signer.sign_to_vec()?)
}

pub fn base64_encode(data: &[u8]) -> String {
    BASE64.encode(data)
}
