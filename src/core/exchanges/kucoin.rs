use async_trait::async_trait;

use crate::{
    config,
    core::{
        traits::ExchangeAPI,
        types::{Asks, Bids, OrderBook},
    },
    utils::{
        base64_encode, encrypt_hmac_sha256, fetch_data, get_current_timestamp, parse_json_glass,
    },
};
use std::{collections::HashSet, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kucoin {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    excluded_trading_pairs: HashSet<String>,
}

impl Kucoin {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
        excluded_trading_pairs: &[&str],
    ) -> Self {
        let mut set: HashSet<String> = HashSet::new();
        for pair in excluded_trading_pairs {
            set.insert(pair.to_string().to_uppercase());
        }
        Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            excluded_trading_pairs: set,
        }
    }
}

#[async_trait]
impl ExchangeAPI for Kucoin {
    fn name(&self) -> &str {
        &self.name
    }
    fn api_key(&self) -> &str {
        &self.api_key
    }
    fn secret_key(&self) -> &str {
        &self.secret_key
    }
    fn base_url(&self) -> &str {
        &self.base_url
    }
    fn is_pair_excluded(&self, quote: &str, base: &str) -> bool {
        self.excluded_trading_pairs.contains(&format!(
            "{}_{}",
            quote.to_uppercase(),
            base.to_uppercase()
        ))
    }
    async fn fetch_order_book(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        //kc-api-key
        let kc_api_key = self.api_key().to_string();

        //kc-api-key
        let timestamp = get_current_timestamp()?;
        let method = "GET";
        let endpoint = format!(
            "{}?symbol={}-{}",
            "/api/v3/market/orderbook/level2",
            base.to_uppercase(),
            quote.to_uppercase()
        );
        let sign = (&timestamp).to_string() + method + &endpoint;
        let encrypted_sign = encrypt_hmac_sha256(&self.secret_key, &sign)?;
        let kc_api_sign = base64_encode(&encrypted_sign);

        // kc-api-timestamp
        let kc_api_timestamp = timestamp;

        // kc-api-passphrase
        let kc_api_passphrase = base64_encode(&encrypt_hmac_sha256(
            &self.secret_key,
            &config::kucoin::PASSPHRASE.to_string(),
        )?);

        //kc-api-version
        let kc_api_version = "3".to_string();

        let result = fetch_data(
            vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("KC-API-KEY".to_string(), kc_api_key),
                ("KC-API-SIGN".to_string(), kc_api_sign),
                ("KC-API-TIMESTAMP".to_string(), kc_api_timestamp),
                ("KC-API-PASSPHRASE".to_string(), kc_api_passphrase),
                ("KC-API-KEY-VERSION".to_string(), kc_api_version),
            ],
            self.base_url().to_string() + "/api/v3/market/orderbook/level2",
            &[("symbol".to_string(), format!("{}-{}", base, quote))],
        )
        .await?;
        if result["code"] != "200000" {
            return Err(format!("Fail fetch {} order book", self.name()).into());
        }

        let raw_asks = &result["data"]["asks"];
        let raw_bids = &result["data"]["bids"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err(format!(
                "Invalid response: 'asks' or 'bids' is not an array - {}",
                result
            )
            .into());
        }
        let formated_asks: Asks = parse_json_glass(&raw_asks)?;
        let formated_bids: Bids = parse_json_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
