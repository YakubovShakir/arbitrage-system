use crate::{
    config::parameters::{MARGIN_INFO_CACHE_TTL_SECS, NETWORKS_CACHE_TTL_SECS},
    core::{
        net::{http::HttpClient, websocket::WebSocketClient},
        types::{api::CacheData, signature_params::SignatureParams},
        utils::crypto::{
            base64_encode, encrypt_hmac_sha256, encrypt_hmac_sha512, hex_encode,
            sha512_with_ring_return_hex,
        },
    },
};
use std::{error::Error, time::Duration};

pub struct CachedConfig {
    pub networks: CacheData,
    pub margin_info: CacheData,
}
impl CachedConfig {
    pub fn new(networks_ttl_secs: u64, margin_info_ttl_secs: u64) -> Self {
        Self {
            networks: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(networks_ttl_secs),
            ),
            margin_info: CacheData::new(
                json::JsonValue::Null,
                Duration::from_secs(margin_info_ttl_secs),
            ),
        }
    }
}
pub struct ExchangeConfig {
    pub name: String,
    pub api_key: String,
    pub secret_key: String,
    pub http_client: HttpClient,
    pub futures_http_client: HttpClient,
    pub websocket_client: Option<WebSocketClient>,
    pub futures_websocket_client: Option<WebSocketClient>,
    pub cached_data: CachedConfig,
}

pub enum Exchange {
    Binance(ExchangeConfig),
    Bybit(ExchangeConfig),
    Bitget(ExchangeConfig),
    Gate(ExchangeConfig),
    Kucoin(ExchangeConfig),
    Mexc(ExchangeConfig),
    Huobi(ExchangeConfig),
    Bitmart(ExchangeConfig),
    Okx(ExchangeConfig),
}
impl Exchange {
    pub fn config(&self) -> &ExchangeConfig {
        match self {
            Exchange::Binance(config) => config,
            Exchange::Bybit(config) => config,
            Exchange::Bitget(config) => config,
            Exchange::Gate(config) => config,
            Exchange::Kucoin(config) => config,
            Exchange::Mexc(config) => config,
            Exchange::Huobi(config) => config,
            Exchange::Bitmart(config) => config,
            Exchange::Okx(config) => config,
        }
    }

    pub fn generate_signature(&self, params: SignatureParams) -> Result<String, Box<dyn Error>> {
        match (self, params) {
            // Binance
            (Self::Binance(cfg), SignatureParams::Binance { query }) => {
                let signature = encrypt_hmac_sha256(&cfg.secret_key, query)?;
                Ok(hex_encode(signature))
            }

            // Bybit
            (
                Self::Bybit(cfg),
                SignatureParams::Bybit {
                    query,
                    timestamp,
                    recv_window,
                },
            ) => {
                let ak = &cfg.api_key;
                let sk = &cfg.secret_key;
                let signature = timestamp.to_owned() + ak + recv_window + query;
                let encrypted = encrypt_hmac_sha256(sk, &signature)?;
                Ok(hex_encode(encrypted))
            }

            // Bitget
            (
                Self::Bitget(cfg),
                SignatureParams::Bitget {
                    query,
                    method,
                    endpoint,
                    timestamp,
                },
            ) => {
                let signature = timestamp.to_owned() + method + endpoint + "?" + query;
                let encrypted = encrypt_hmac_sha256(&cfg.secret_key, &signature)?;
                Ok(base64_encode(&encrypted))
            }

            // Gate.io
            (
                Self::Gate(cfg),
                SignatureParams::Gate {
                    query,
                    method,
                    endpoint,
                    timestamp,
                    json_body,
                },
            ) => {
                let hashed_payload = sha512_with_ring_return_hex(json_body);
                let prepared_str = format!(
                    "{}\n{}\n{}\n{}\n{}",
                    method, endpoint, query, hashed_payload, timestamp
                );
                let signed = encrypt_hmac_sha512(&cfg.secret_key, &prepared_str)?;
                Ok(hex_encode(signed))
            }

            // Kucoin
            (
                Self::Kucoin(cfg),
                SignatureParams::Kucoin {
                    query,
                    method,
                    endpoint,
                    timestamp,
                },
            ) => {
                let signature = timestamp.to_owned() + method + endpoint + "?" + query;
                let encrypted = encrypt_hmac_sha256(&cfg.secret_key, &signature)?;
                Ok(base64_encode(&encrypted))
            }

            // Mexc
            (Self::Mexc(cfg), SignatureParams::Mexc { query, string_body }) => {
                let signature = query.to_string() + string_body;
                let encrypted = encrypt_hmac_sha256(&cfg.secret_key, &signature)?;
                Ok(hex_encode(encrypted))
            }
            // Huobi
            (
                Self::Huobi(cfg),
                SignatureParams::Huobi {
                    method,
                    host,
                    path,
                    params,
                },
            ) => {
                let mut urlencoded_params = params
                    .iter()
                    .map(|(key, value)| (urlencoding::encode(key), urlencoding::encode(value)))
                    .collect::<Vec<_>>();

                urlencoded_params.sort_by(|a, b| a.0.cmp(&b.0));
                let query = urlencoded_params
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<_>>()
                    .join("&");

                let prepared_str = format!("{}\n{}\n{}\n{}", method, host, path, query);
                let encrypted = encrypt_hmac_sha256(&cfg.secret_key, &prepared_str)?;
                let signed = base64_encode(&encrypted);
                Ok(signed)
            }
            (
                Self::Okx(cfg),
                SignatureParams::Okx {
                    timestamp,
                    method,
                    request_path,
                    body,
                },
            ) => {
                let pre_hash = format!("{}{}{}{}", timestamp, method, request_path, body);
                let encrypted = encrypt_hmac_sha256(&cfg.secret_key, &pre_hash)?;
                let encoded = base64_encode(&encrypted);
                Ok(encoded)
            }
            _ => Err(format!(
                "Cannot generate Signature - method not implemented for this Exchange",
            )
            .into()),
        }
    }
}
