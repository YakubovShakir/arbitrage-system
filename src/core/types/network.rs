use crate::core::utils::{parse_json_as_f64, parse_json_as_str};
use json::JsonValue;

#[derive(Debug, Clone)]
pub struct Network {
    pub name: String,
    pub full_name: String,
    pub coin_name: String,
    pub withdraw_fee: Option<f64>,
    pub contract_address: Option<String>,
    pub memo: Option<String>,
    pub deposit_address: Option<String>,
}

impl Network {
    pub fn new(
        name: String,
        full_name: String,
        coin_name: String,
        withdraw_fee: Option<f64>,
        contract_address: Option<String>,
        memo: Option<String>,
        deposit_address: Option<String>,
    ) -> Self {
        let contract_address = contract_address.filter(|addr| !addr.is_empty());
        let memo = memo.filter(|m| !m.is_empty());
        let deposit_address = deposit_address.filter(|addr| !addr.is_empty());

        Network {
            name,
            full_name,
            coin_name,
            withdraw_fee,
            contract_address,
            memo: memo,
            deposit_address,
        }
    }
    pub fn create_test() -> Network {
        Network::new(
            "Test Network name".to_string(),
            "Test Network full_name".to_string(),
            "Test Network coin".to_string(),
            None,
            None,
            None,
            None,
        )
    }
    pub fn parse_json(
        name: &JsonValue,
        full_name: &JsonValue,
        coin_name: String,
        withdraw_fee: Option<&JsonValue>,
        contract_address: &JsonValue,
        memo: Option<JsonValue>,
        deposit_address: Option<JsonValue>,
    ) -> Result<Network, Box<dyn std::error::Error>> {
        let name = parse_json_as_str(name)?;
        let full_name = parse_json_as_str(full_name)?;
        let withdraw_fee = withdraw_fee.and_then(|fee| parse_json_as_f64(fee).ok());
        let contract_address = parse_json_as_str(contract_address).ok();
        let memo = memo.and_then(|memo| parse_json_as_str(&memo).ok());
        let deposit_address = deposit_address.and_then(|addr| parse_json_as_str(&addr).ok());

        Ok(Self {
            name,
            full_name,
            coin_name,
            withdraw_fee,
            contract_address,
            memo,
            deposit_address,
        })
    }
}
