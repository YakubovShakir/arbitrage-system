use async_trait::async_trait;
use json::JsonValue;
use log::debug;
use std::time::Duration;

use crate::{
    config::parameters::{
        BITGET_MARGIN_INFO_HTTP_TIMEOUT_SECONDS, DEBUG_CODE, GATE_MARGIN_INFO_HTTP_TIMEOUT_SECONDS,
        HUOBI_MARGIN_INFO_HTTP_TIMEOUT_SECONDS, RESET_CODE,
    },
    core::{
        traits::exchange_service::MarginInfoService,
        types::{API, TradingPair, exchanges::Exchange, signature_params::SignatureParams},
        utils::{
            get_current_timestamp, get_timestamp_iso_8601,
            json_utils::{find_value_from_json_key, parse_json_as_bool, parse_json_as_str},
        },
    },
};

#[async_trait]
impl MarginInfoService for Exchange {
    async fn borrowable(&self, pair: &TradingPair) -> Result<bool, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetMarginInfo.endpoint(self) else {
            // return Err(format!(
            //     "Error: GetMarginInfo endpoint is not set for the exchange {}",
            //     self.config().name
            // )
            // .into());
            return Ok(false);
        };

        let cache_data = self.config().cached_data.margin_info.get().await;

        if cache_data.is_none() {
            debug!(target:"debug_module", "Cache not given for margin_info {}", self.config().name);
        }

        let raw_borrowable: JsonValue = match self {
            Exchange::Binance(cfg) => {
                let headers = &[("X-MBX-APIKEY", cfg.api_key.as_str())];
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let response = cfg
                            .http_client
                            .get(endpoint, None, Some(headers), None)
                            .await?;

                        cfg.cached_data.margin_info.set(response.clone()).await;
                        response
                    }
                };

                let mut borrowable = JsonValue::Boolean(false);

                for asset in data.members() {
                    let Ok(asset_name) = parse_json_as_str(&asset["assetName"]) else {
                        continue;
                    };
                    if asset_name != pair.base {
                        continue;
                    }
                    borrowable = find_value_from_json_key(&asset, &["isBorrowable"])?;
                }
                borrowable
            }

            Exchange::Bybit(cfg) => {
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let response = cfg.http_client.get(endpoint, None, None, None).await?;
                        if response["result"].has_key("vipCoinList") {
                            cfg.cached_data.margin_info.set(response.clone()).await;
                        };
                        response
                    }
                };

                if data["result"] == JsonValue::Null {
                    JsonValue::Boolean(false)
                } else {
                    let vip_coin_list =
                        find_value_from_json_key(&data, &["result", "vipCoinList"])?;
                    let vip_level = 0;
                    let vip_level = vip_coin_list
                        .members()
                        .nth(vip_level)
                        .ok_or("vipCoinList is is empty")?;
                    let list = find_value_from_json_key(&vip_level, &["list"])?;
                    let mut borrowable = JsonValue::Boolean(false);

                    for item in list.members() {
                        let Ok(asset_name) = parse_json_as_str(&item["currency"]) else {
                            continue;
                        };
                        if asset_name != pair.base {
                            continue;
                        };
                        borrowable = find_value_from_json_key(&item, &["borrowable"])?;
                    }
                    borrowable
                }
            }

            Exchange::Bitget(cfg) => {
                let timestamp = get_current_timestamp()?;
                let sign = self.generate_signature(SignatureParams::Bitget {
                    query: "",
                    method: "GET",
                    endpoint,
                    timestamp: &timestamp,
                })?;

                let headers = &[("ACCESS-KEY", cfg.api_key.as_str()), ("ACCESS-SIGN", &sign)];

                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                None,
                                Some(headers),
                                Some(Duration::from_secs(BITGET_MARGIN_INFO_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        cfg.cached_data.margin_info.set(response.clone()).await;

                        response
                    }
                };
                let data = find_value_from_json_key(&data, &["data"])?;
                if !data.is_array() {
                    return Err(
                        format!("{} Invalid response: 'data'is not an array", cfg.name).into(),
                    );
                }
                let mut result = JsonValue::Boolean(false);
                let symbol = pair.base.to_string() + &pair.quote;
                for item in data.members() {
                    if item["symbol"] != symbol.as_str() {
                        continue;
                    };
                    result = find_value_from_json_key(&item, &["isIsolatedBaseBorrowable"])?;
                }
                result
            }

            Exchange::Gate(cfg) => {
                // let t = get_current_timestamp_secs()?;
                // let auth_headers = self.get_auth_headers("GET", endpoint, None, None, t)?;
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                None,
                                None,
                                Some(Duration::from_secs(GATE_MARGIN_INFO_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        if response.is_empty() {
                            return Err(format!(
                                "{} Invalid response: 'items' is not an array",
                                cfg.name
                            )
                            .into());
                        }
                        cfg.cached_data.margin_info.set(response.clone()).await;
                        response
                    }
                };

                let mut result = JsonValue::Boolean(false);
                for item in data.members() {
                    let Ok(pair_name) = parse_json_as_str(&item["currency_pair"]) else {
                        continue;
                    };

                    if pair_name != format!("{}_{}", pair.base, pair.quote) {
                        continue;
                    }
                    result = JsonValue::Boolean(true);
                }
                result
            }
            Exchange::Kucoin(cfg) => {
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let response = cfg
                            .http_client
                            .get(&endpoint.to_string(), None, None, None)
                            .await?;

                        if response["code"] == "200000" {
                            cfg.cached_data.margin_info.set(response.clone()).await;
                        }
                        response
                    }
                };
                let mut borrowable = JsonValue::Boolean(false);

                for item in data["data"].members() {
                    let Ok(asset_name) = parse_json_as_str(&item["currency"]) else {
                        continue;
                    };
                    if asset_name != pair.base {
                        continue;
                    }
                    borrowable = find_value_from_json_key(&item, &["isMarginEnabled"])?;
                }
                borrowable
            }
            Exchange::Mexc(_) => JsonValue::Boolean(false),
            Exchange::Huobi(cfg) => {
                let symbols = format!("{}{}", pair.base, pair.quote).to_lowercase();
                let timestamp = get_timestamp_iso_8601()?;

                // PARAMETERS
                let mut parameters = vec![
                    ("AccessKeyId", cfg.api_key.as_str()),
                    ("order-id", "124432"), // ВАЖНО: Для GET-запросов все параметры включаются
                    ("SignatureMethod", "HmacSHA256"),
                    ("SignatureVersion", "2"),
                    ("Timestamp", &timestamp),
                    ("symbols", symbols.as_str()),
                ];

                let signature = self.generate_signature(SignatureParams::Huobi {
                    method: "GET",
                    host: "api.huobi.pro",
                    path: endpoint,
                    params: &parameters,
                })?;

                parameters.push(("Signature", &signature));

                let headers = &[("Content-Type", "application/json")];
                let res = cfg
                    .http_client
                    .get(
                        endpoint,
                        Some(&parameters),
                        Some(headers),
                        Some(Duration::from_secs(HUOBI_MARGIN_INFO_HTTP_TIMEOUT_SECONDS)),
                    )
                    .await?;

                let data = find_value_from_json_key(&res, &["data"])?;
                if data.len() == 1 {
                    JsonValue::Boolean(true)
                } else {
                    JsonValue::Boolean(false)
                }
            }
            Exchange::Bitmart(_) => JsonValue::Boolean(false),
            Exchange::Okx(_) => JsonValue::Boolean(false),
        };

        let borrowable = parse_json_as_bool(&raw_borrowable)?;
        Ok(borrowable)
    }
}
