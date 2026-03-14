use std::env;

use async_trait::async_trait;
use json::JsonValue;

use crate::core::{
    traits::exchange_service::FuturesOrderBookService,
    types::{
        API, Asks, Bids, OrderBook, Price, Quantity, exchanges::Exchange,
        signature_params::SignatureParams,
    },
    utils::{
        crypto::{base64_encode, encrypt_hmac_sha256},
        get_current_timestamp,
        json_utils::{find_value_from_json_key, parse_json_as_f64},
    },
};

#[async_trait]
impl FuturesOrderBookService for Exchange {
    async fn futures_orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetFuturesOrderBook.endpoint(self) else {
            return Err(format!(
                "Error: GetOrderBook endpoint is not set for the exchange {}",
                self.config().name
            )
            .into());
        };
        let (raw_asks, raw_bids) = match self {
            Exchange::Binance(cfg) => {
                let symbol = base.to_string() + quote;
                let query = &[("symbol", symbol.as_str()), ("limit", "500")];
                let headers = &[("X-MBX-APIKEY", cfg.api_key.as_str())];
                let response = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Bybit(cfg) => {
                let order_book_limit = "500";
                let recv_window = "5000";
                let t = get_current_timestamp()?;
                let query_string = format!(
                    "category={}&symbol={}&limit={}",
                    "linear",
                    base.to_string() + quote,
                    order_book_limit,
                );
                let signature = self.generate_signature(SignatureParams::Bybit {
                    query: &query_string,
                    timestamp: &t,
                    recv_window,
                })?;

                let headers = &[
                    ("X-BAPI-API-KEY", cfg.api_key.as_str()),
                    ("X-BAPI-RECV-WINDOW", recv_window),
                    ("X-BAPI-TIMESTAMP", &t),
                    ("X-BAPI-SIGN", &signature),
                ];
                let symbol = base.to_string() + quote;
                let query = &[
                    ("category", "spot"),
                    ("symbol", symbol.as_str()),
                    ("limit", order_book_limit),
                ];
                let response = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["result", "a"])?;
                let raw_bids = find_value_from_json_key(&response, &["result", "b"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Bitget(cfg) => {
                let symbol = base.to_string() + quote;
                let query = &[
                    ("symbol", symbol.as_str()),
                    ("limit", "150"),
                    ("productType", "USDT-FUTURES"),
                ];
                let headers = &[("Content-Type", "application/json")];
                let response = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Mexc(cfg) => {
                let endpoint = format!("{}/{}_{}", endpoint, base, quote);
                let headers = &[
                    ("X-MEXC-APIKEY", cfg.api_key.as_str()),
                    ("Content-Type", "application/json"),
                ];
                let response = cfg
                    .futures_http_client
                    .get(&endpoint, None, Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Kucoin(cfg) => {
                let endpoint = &format!("{}{}", endpoint, 100);
                let symbol = format!("{}{}M", base.to_uppercase(), quote.to_uppercase());
                let queries = &[("symbol", symbol.as_str())];
                let headers = &[("Content-Type", "application/json")];
                let response = cfg
                    .futures_http_client
                    .get(endpoint, Some(queries), Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Gate(cfg) => {
                let symbol = format!("{}_{}", base, quote);
                let query = &[("contract", symbol.as_str())];
                let headers = &[
                    ("Accept", "application/json"),
                    ("Content-Type", "application/json"),
                ];
                let response = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["asks"])?;
                let mut normalized_asks = JsonValue::new_array();
                for ask in raw_asks.members() {
                    let _ = normalized_asks.push(vec![ask["p"].clone(), ask["s"].clone()]);
                }

                let raw_bids = find_value_from_json_key(&response, &["bids"])?;
                let mut normalized_bids = JsonValue::new_array();
                for bid in raw_bids.members() {
                    let _ = normalized_bids.push(vec![bid["p"].clone(), bid["s"].clone()]);
                }
                (normalized_asks, normalized_bids)
            }
            Exchange::Huobi(cfg) => {
                let symbol = format!("{}-{}", base, quote);
                let headers = &[("Content-Type", "application/json")];
                let query = &[("contract_code", symbol.as_str()), ("type", "step0")];
                let res = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;

                let raw_asks = find_value_from_json_key(&res, &["tick", "asks"])?;
                let raw_bids = find_value_from_json_key(&res, &["tick", "bids"])?;

                (raw_asks, raw_bids)
            }
            Exchange::Bitmart(cfg) => {
                let symbol = format!("{}{}", base, quote);
                let query = &[("symbol", symbol.as_str())];

                let res = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), None, None)
                    .await?;
                let raw_asks = find_value_from_json_key(&res, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&res, &["data", "bids"])?;

                (raw_asks, raw_bids)
            }
            Exchange::Okx(cfg) => {
                let symbol = format!("{}-{}-SWAP", base, quote);
                let query = &[("instId", symbol.as_str()), ("sz", "400")];

                let res = cfg
                    .futures_http_client
                    .get(endpoint, Some(query), None, None)
                    .await?;
                if !res["data"].is_array() || res["data"].len() == 0 {
                    (JsonValue::Null, JsonValue::Null)
                } else {
                    let data = &res["data"][0];
                    let raw_asks = find_value_from_json_key(&data, &["asks"])?;
                    let raw_bids = find_value_from_json_key(&data, &["bids"])?;

                    (raw_asks, raw_bids)
                }
            }
        };

        if raw_asks.is_empty() || raw_bids.is_empty() {
            return Err("Invalid response: 'asks' or 'bids' is empty".into());
        }
        let formated_asks: Asks = parse_glass(&raw_asks)?;
        let formated_bids: Bids = parse_glass(&raw_bids)?;
        Ok((formated_asks, formated_bids))
    }
}

// В нулевом индексе пары должна быть цена, а в первом количество
pub fn parse_glass(glass: &JsonValue) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for order_level in glass.members() {
        let price: Price = parse_json_as_f64(&order_level[0])?;
        let quantity: Quantity = parse_json_as_f64(&order_level[1])?;
        result.push((price, quantity));
    }
    Ok(result)
}
