use super::client::get_client;
use crate::core::{Glass, KeyValueString, Price, Quantity};
use json::JsonValue;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

pub fn calc_average_price(quote_limit: &f64, glass: &Glass) -> Option<f64> {
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

pub async fn fetch_data(
    headers: Vec<KeyValueString>,
    url: String,
    query_params: &[KeyValueString],
) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut headers_map: HeaderMap = HeaderMap::new();

    for header in headers {
        headers_map.insert(
            HeaderName::from_str(header.0.as_str())?,
            HeaderValue::from_str(header.1.as_str())?,
        );
    }

    Ok(json::parse(
        &get_client()
            .await
            .get(url)
            .headers(headers_map)
            .query(query_params)
            .send()
            .await?
            .text()
            .await?,
    )?)
}

// В нулевом индексе пары должна быть цена, а в первом количество
pub fn parse_json_glass(glass: &JsonValue) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
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
