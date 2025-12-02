use async_trait::async_trait;

use crate::core::{
    net::http::HttpClient,
    traits::{ExchangeService, ExchangeStatic},
    types::{
        Asks, Bids, OrderBook, TradingPairs,
        structs::{Network, PriceData, TradingPair},
    },
    utils::{
        encrypt_hmac_sha512, get_current_timestamp_secs, hex_encode, parse_json_as_bool,
        parse_json_as_f64, parse_json_as_str, parse_string_typed_glass,
        sha512_with_ring_return_hex,
    },
};
use std::{collections::HashMap, error::Error};

pub struct Gate {
    name: String,
    api_key: String,
    secret_key: String,
    base_url: String,
    http_client: HttpClient,
}

impl Gate {
    pub fn new(
        name: &str,
        api_key: &str,
        secret_key: &str,
        base_url: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.to_string(),
            http_client: HttpClient::new(base_url)?,
        })
    }

    fn get_sign(
        &self,
        method: &str,
        endpoint: &str,
        query_string: Option<&str>,
        body_payload_in_json_format: Option<&str>,
        timestamp_in_secs: String,
    ) -> Result<String, Box<dyn Error>> {
        let query_string = query_string.unwrap_or("");
        let hashed_payload = sha512_with_ring_return_hex(body_payload_in_json_format.unwrap_or(""));

        let prepared_str = format!(
            "{}\n{}\n{}\n{}\n{}",
            method, endpoint, query_string, hashed_payload, timestamp_in_secs
        );
        let signed = encrypt_hmac_sha512(&self.secret_key, &prepared_str)?;

        Ok(hex_encode(signed))
    }

    fn get_auth_headers(
        &self,
        method: &str,
        endpoint: &str,
        query_string: Option<&str>,
        body_payload_in_json_format: Option<&str>,
        timestamp_in_secs: String,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let sign = self.get_sign(
            method,
            endpoint,
            query_string,
            body_payload_in_json_format,
            timestamp_in_secs.clone(),
        )?;
        Ok(vec![
            ("Accept".to_string(), "application/json".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("KEY".to_string(), self.api_key.to_string()),
            ("Timestamp".to_string(), timestamp_in_secs),
            ("SIGN".to_string(), sign),
        ])
    }
}

impl ExchangeStatic for Gate {
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
}
#[async_trait]
impl ExchangeService for Gate {
    async fn fetch_tickers(&self) -> Result<TradingPairs, Box<dyn std::error::Error>> {
        let tickers = self
            .http_client
            .get(
                "/api/v4/spot/tickers",
                None,
                Some(&[
                    ("Accept".to_string(), "application/json".to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
            )
            .await?;

        if tickers.is_empty() {
            return Err(
                format!("{} Invalid response: 'tickers' is not an array", self.name).into(),
            );
        }
        let mut trading_pairs: TradingPairs = HashMap::new();

        for ticker in tickers.members() {
            let Some(pair) =
                TradingPair::from_str_with_separator(&ticker["currency_pair"].to_string(), '_')
            else {
                continue;
            };
            let (Ok(lowest_ask), Ok(highest_bid)) = (
                parse_json_as_f64(&ticker["lowest_ask"]),
                parse_json_as_f64(&ticker["highest_bid"]),
            ) else {
                continue;
            };

            let price_data = PriceData::new(
                (self.name.clone(), highest_bid),
                (self.name.clone(), lowest_ask),
            );

            trading_pairs.insert(pair, price_data);
        }
        // println!("Parsed trading pairs - {:?}", trading_pairs);
        Ok(trading_pairs)
    }
    async fn is_margin_available(
        &self,
        trading_pair_name: &TradingPair,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let endpoint = "/api/v4/margin/uni/currency_pairs";
        // let t = get_current_timestamp_secs()?;
        // let auth_headers = self.get_auth_headers("GET", endpoint, None, None, t)?;
        let items = self.http_client.get(endpoint, None, None).await?;

        if items.is_empty() {
            return Err(format!("{} Invalid response: 'items' is not an array", self.name).into());
        }
        for item in items.members() {
            let Ok(pair_name) = parse_json_as_str(&item["currency_pair"]) else {
                continue;
            };

            if pair_name != format!("{}_{}", trading_pair_name.base, trading_pair_name.quote) {
                continue;
            }
            return Ok(true);
        }

        Ok(false)
    }
    async fn fetch_networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let chains = self
            .http_client
            .get(
                "/api/v4/wallet/currency_chains",
                Some(&[("currency".to_string(), coin.to_string())]),
                Some(&[
                    ("Accept".to_string(), "application/json".to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
            )
            .await?;

        if chains.is_empty() {
            return Err(format!("{} Invalid response: 'chains' is not an array", self.name).into());
        }

        let mut fetched_networks: Vec<Network> = Vec::new();

        for chain in chains.members() {
            if let (Ok(is_disabled), Ok(deposit_disabled), Ok(withdraw_disabled)) = (
                parse_json_as_bool(&chain["is_disabled"]),
                parse_json_as_bool(&chain["is_deposit_disabled"]),
                parse_json_as_bool(&chain["is_withdraw_disabled"]),
            ) {
                if is_disabled || deposit_disabled || withdraw_disabled {
                    continue;
                }
            } else {
                continue;
            }

            if let Ok(network) = Network::parse_json(
                &chain["chain"],
                &chain["name_en"],
                coin.to_string(),
                None,
                &chain["contract_address"],
                None,
                None,
            ) {
                fetched_networks.push(network);
            };
        }

        Ok(fetched_networks)
    }

    async fn fetch_orderbook(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn Error>> {
        let result = self
            .http_client
            .get(
                "/api/v4/spot/order_book",
                Some(&[(
                    "currency_pair".to_string(),
                    format!("{}_{}", base.to_string(), &quote.to_string()),
                )]),
                Some(&[
                    ("Accept".to_string(), "application/json".to_string()),
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
            )
            .await?;

        if !result.has_key("asks") || !result.has_key("bids") {
            return Err(format!(
                "Invalid response {} : missing 'asks' or 'bids' field",
                self.name()
            )
            .into());
        }

        let raw_asks = &result["asks"];
        let raw_bids = &result["bids"];

        if !raw_asks.is_array() || !raw_bids.is_array() {
            return Err("Invalid response: 'asks' or 'bids' is not an array".into());
        }
        let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
        let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;

        Ok((formated_asks, formated_bids))
    }
}
