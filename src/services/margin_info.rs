use async_trait::async_trait;
use json::JsonValue;

use crate::core::{
    traits::exchange_service::MarginInfoService,
    types::{API, TradingPair, exchanges::Exchange, signature_params::SignatureParams},
    utils::{
        find_value_from_json_key, get_current_timestamp, parse_json_as_bool, parse_json_as_str,
    },
};

#[async_trait]
impl MarginInfoService for Exchange {
    async fn borrowable(&self, pair: &TradingPair) -> Result<bool, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetMarginInfo.endpoint(self) else {
            return Err(format!(
                "Error: GetMarginInfo endpoint is not set for the exchange {}",
                self.config().name
            )
            .into());
        };

        let raw_borrowable: JsonValue = match self {
            Exchange::Binance(cfg) => {
                let query = &[("asset", pair.base.as_str())];
                let headers = &[("X-MBX-APIKEY", cfg.api_key.as_str())];
                let response = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                find_value_from_json_key(&response[0], &["isBorrowable"])?
            }

            Exchange::Bybit(cfg) => {
                let query = &[("currency", pair.base.as_str())];
                let response = cfg.http_client.get(endpoint, Some(query), None).await?;
                let vip_list = find_value_from_json_key(&response, &["result", "vipCoinList"])?;
                let vip_level = 0;
                let vip_list_level = vip_list
                    .members()
                    .nth(vip_level)
                    .ok_or("vipCoinList is is empty")?;
                let list_item = vip_list_level["list"]
                    .members()
                    .nth(0)
                    .ok_or("list is empty")?;
                find_value_from_json_key(&list_item, &["borrowable"])?
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
                let response = cfg.http_client.get(endpoint, None, Some(headers)).await?;
                let data = find_value_from_json_key(&response, &["data"])?;
                if !data.is_array() {
                    return Err(
                        format!("{} Invalid response: 'data'is not an array", cfg.name).into(),
                    );
                }

                let mut result = JsonValue::Null;
                for item in data.members() {
                    if item["baseCoin"] != pair.base {
                        continue;
                    };
                    result = find_value_from_json_key(&item, &["isBorrowable"])?;
                }
                result
            }

            Exchange::Gate(cfg) => {
                // let t = get_current_timestamp_secs()?;
                // let auth_headers = self.get_auth_headers("GET", endpoint, None, None, t)?;
                let items = cfg.http_client.get(endpoint, None, None).await?;
                if items.is_empty() {
                    return Err(
                        format!("{} Invalid response: 'items' is not an array", cfg.name).into(),
                    );
                }

                let mut result = JsonValue::Null;
                for item in items.members() {
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
                let query = &[("currency", pair.base.as_str())];
                let response = cfg.http_client.get(endpoint, Some(query), None).await?;
                find_value_from_json_key(&response, &["data", "isMarginEnabled"])?
            }
            Exchange::Mexc(cfg) => JsonValue::Boolean(false),
        };

        let borrowable = parse_json_as_bool(&raw_borrowable)?;
        Ok(borrowable)
    }
}
