use async_trait::async_trait;

use crate::core::{
    traits::exchange_service::OrderBookService,
    types::{API, Asks, Bids, OrderBook, exchanges::Exchange},
    utils::{find_value_from_json_key, parse_string_typed_glass},
};

#[async_trait]
impl OrderBookService for Exchange {
    async fn get(&self, base: &str, quote: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::endpoint(&API::GetOrderBook, self) else {
            return Err(format!(
                "Error: GetOrderBook endpoint is not set for the exchange {}",
                self.get_name()
            )
            .into());
        };

        match self {
            Exchange::Binance(config) => {
                let query = &[
                    ("symbol", format!("{}{}", base, quote).as_str()),
                    ("limit", "500"),
                ];
                let headers = &[("X-MBX-APIKEY", self.get_api_key())];
                let response = self
                    .get_http_client()
                    .get(endpoint, Some(query), Some(headers))
                    .await?;
                let raw_asks = find_value_from_json_key(&response, &["asks"])?;
                let raw_bids = find_value_from_json_key(&response, &["bids"])?;
                if !raw_asks.is_empty() || !raw_bids.is_empty() {
                    return Err("Invalid response: 'asks' or 'bids' is empty".into());
                }
                let formated_asks: Asks = parse_string_typed_glass(&raw_asks)?;
                let formated_bids: Bids = parse_string_typed_glass(&raw_bids)?;
                Ok((formated_asks, formated_bids))
            }
        }
    }
}
