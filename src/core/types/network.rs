use crate::config::get_map;
use log::error;

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub full_name: String,
    pub contract_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub config: NetworkConfig,
    pub network_type: String,
}
impl Network {
    pub fn parse(name: String, full_name: String) -> Option<Self> {
        let network_type = get_map().get(&name.to_uppercase()).map(|s| s.as_str());

        match network_type {
            Some(network_type) => {
                let config = NetworkConfig {
                    name,
                    full_name,
                    contract_address: None,
                };

                Some(Self {
                    config,
                    network_type: network_type.to_string(),
                })
            }
            None => {
                error!(target: "unknown_networks_module", "Не удалось распознать сеть name: {} full_name: {}",name, full_name);
                None
            }
        }
    }

    pub fn set_contract(&mut self, address: String) -> &Self {
        self.config.contract_address = Some(address);
        self
    }
    pub fn make_withdraw_type(self, withdraw_fee: Option<f64>) -> WithdrawNetwork {
        WithdrawNetwork {
            base: self,
            withdraw_fee,
        }
    }
    pub fn make_deposit_type(
        self,
        memo: Option<String>,
        deposit_address: Option<String>,
        number_of_confirmation: Option<u64>,
    ) -> DepositNetwork {
        DepositNetwork {
            base: self,
            memo,
            deposit_address,
            number_of_confirmation,
        }
    }
    pub fn create_test() -> Network {
        let config: NetworkConfig = NetworkConfig {
            name: "BTC".to_string(),
            full_name: "Bitcoin".to_string(),
            contract_address: None,
        };
        Network {
            config,
            network_type: "Bitcoin".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawNetwork {
    pub base: Network,
    pub withdraw_fee: Option<f64>,
}
impl WithdrawNetwork {
    pub fn set_withdraw_fee(&mut self, fee: f64) -> &Self {
        self.withdraw_fee = Some(fee);
        self
    }
}
impl DepositNetwork {
    pub fn set_number_of_confirm(&mut self, number: u64) -> &Self {
        self.number_of_confirmation = Some(number);
        self
    }
    pub fn set_deposit_addresss(&mut self, address: String) -> &Self {
        self.deposit_address = Some(address);
        self
    }
    pub fn set_memo(&mut self, memo: String) -> &Self {
        self.memo = Some(memo);
        self
    }
}

#[derive(Debug, Clone)]
pub struct DepositNetwork {
    pub base: Network,
    pub memo: Option<String>,
    pub deposit_address: Option<String>,
    pub number_of_confirmation: Option<u64>,
}
