use async_trait::async_trait;
use json::JsonValue;

use crate::{
    config,
    core::{
        traits::exchange_service::OrderBookService,
        types::{
            API, Asks, Bids, OrderBook, Price, Quantity, exchanges::Exchange,
            signature_params::SignatureParams,
        },
        utils::{
            base64_encode, encrypt_hmac_sha256, find_value_from_json_key, get_current_timestamp,
            parse_json_as_f64,
        },
    },
};

#[async_trait]
impl OrderBookService for Exchange {
    async fn orderbook(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<OrderBook, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetOrderBook.endpoint(self) else {
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
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
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
                    "spot",
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
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["result", "a"])?;
                let raw_bids = find_value_from_json_key(&response, &["result", "b"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Bitget(cfg) => {
                let symbol = (base.to_string() + quote).to_lowercase();
                let query = &[
                    ("symbol", symbol.as_str()),
                    ("limit", "150"),
                    ("type", "step0"),
                ];
                let headers = &[("Content-Type", "application/json")];
                let response = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Mexc(cfg) => {
                let symbol = base.to_string() + quote;
                let query = &[("symbol", symbol.as_str()), ("limit", "500")];
                let headers = &[
                    ("X-MEXC-APIKEY", cfg.api_key.as_str()),
                    ("Content-Type", "application/json"),
                ];
                let response = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Kucoin(cfg) => {
                let query = format!("symbol={}-{}", base.to_uppercase(), quote.to_uppercase());
                let t = get_current_timestamp()?;
                let signature = self.generate_signature(SignatureParams::Kucoin {
                    query: &query,
                    method: "GET",
                    endpoint,
                    timestamp: &t,
                })?;

                // kc-api-passphrase
                let kc_api_passphrase = base64_encode(&encrypt_hmac_sha256(
                    &cfg.secret_key,
                    &config::kucoin::PASSPHRASE.to_string(),
                )?);
                let symbol = format!("{}-{}", base.to_uppercase(), quote.to_uppercase());
                let queries = &[("symbol", symbol.as_str())];
                let headers = &[
                    ("Content-Type", "application/json"),
                    ("KC-API-KEY", cfg.api_key.as_str()),
                    ("KC-API-SIGN", &signature),
                    ("KC-API-TIMESTAMP", &t),
                    ("KC-API-PASSPHRASE", &kc_api_passphrase),
                    ("KC-API-KEY-VERSION", "3"),
                ];
                let response = cfg
                    .http_client
                    .get(endpoint, Some(queries), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["data", "asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["data", "bids"])?;
                (raw_asks, raw_bids)
            }
            Exchange::Gate(cfg) => {
                let symbol = format!("{}_{}", base, quote);
                let query = &[("currency_pair", symbol.as_str())];
                let headers = &[
                    ("Accept", "application/json"),
                    ("Content-Type", "application/json"),
                ];
                let response = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["bids"])?;
                (raw_asks, raw_bids)
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
