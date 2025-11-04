use async_trait::async_trait;

use crate::{
    core::{
        traits::ExchangeAPI,
        types::{Asks, Bids, OrderBook},
    },
    utils::{fetch_data, parse_json_glass},
};
use std::{collections::HashSet, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitget {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    excluded_trading_pairs: HashSet<String>,
}

impl Bitget {
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
impl ExchangeAPI for Bitget {
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
        let result = fetch_data(
            vec![("Content-Type".to_string(), "application/json".to_string())],
            self.base_url().to_string() + "/api/v2/spot/market/orderbook",
            &[
                (
                    "symbol".to_string(),
                    (base.to_string() + quote).to_lowercase(),
                ),
                ("limit".to_string(), "150".to_string()),
                ("type".to_string(), "step0".to_string()),
            ],
        )
        .await?;
        if result["msg"] != "success" {
            return Err(format!("Fail fetch {} order book", self.name()).into());
        }

        let raw_asks = &result["data"]["asks"];
        let raw_bids = &result["data"]["bids"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }
        let formated_asks: Asks = parse_json_glass(&raw_asks)?;
        let formated_bids: Bids = parse_json_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
