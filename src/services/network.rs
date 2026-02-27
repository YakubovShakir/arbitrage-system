use std::{env, time::Duration};

use crate::{
    config::parameters::{
        BINANCE_NETWORKS_HTTP_TIMEOUT_SECONDS, BITGET_NETWORKS_HTTP_TIMEOUT_SECONDS, ERROR_CODE,
        MEXC_NETWORKS_HTTP_TIMEOUT_SECONDS, RESET_CODE,
    },
    core::{
        traits::exchange_service::NetworkService,
        types::{
            API, ExchangeNetworks, Network,
            exchanges::Exchange,
            network::{DepositNetwork, WithdrawNetwork},
            signature_params::SignatureParams,
        },
        utils::{
            get_current_timestamp, get_timestamp_iso_8601, get_timestamp_iso_8601_with_ms,
            json_utils::{
                find_value_from_json_key, parse_json_as_bool, parse_json_as_f64, parse_json_as_str,
                parse_json_as_u64,
            },
        },
    },
};
use async_trait::async_trait;
use log::{debug, error};

#[async_trait]
impl NetworkService for Exchange {
    async fn networks(&self, coin: &str) -> Result<ExchangeNetworks, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetNetworks.endpoint(self) else {
            return Err(format!(
                "Error: GetNetworks endpoint is not set for the exchange {}",
                self.config().name
            )
            .into());
        };
        let mut withdraw_networks: Vec<WithdrawNetwork> = Vec::new();
        let mut deposit_networks: Vec<DepositNetwork> = Vec::new();

        let cache_data = self.config().cached_data.networks.get().await;
        if cache_data.is_none() {
            debug!(target:"debug_module", "Cache is None for {}. Call API.. ", self.config().name);
        }
        match self {
            Exchange::Binance(cfg) => {
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let timestamp = get_current_timestamp()?;
                        let query_string = format!("coin={}&timestamp={}", coin, timestamp);
                        let signature = self.generate_signature(SignatureParams::Binance {
                            query: &query_string,
                        })?;

                        let query = &[
                            ("coin", coin),
                            ("timestamp", &timestamp),
                            ("signature", &signature),
                        ];

                        let headers = &[("X-MBX-APIKEY", cfg.api_key.as_str())];

                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                Some(query),
                                Some(headers),
                                Some(Duration::from_secs(BINANCE_NETWORKS_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        if response.is_array() {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data.members() {
                    if item["coin"] != coin {
                        continue;
                    }

                    let networks = &item["networkList"];
                    for network in networks.members() {
                        let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                            parse_json_as_bool(&network["withdrawEnable"]),
                            parse_json_as_bool(&network["depositEnable"]),
                        ) else {
                            continue;
                        };
                        if !withdraw_enabled && !deposit_enabled {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&network["network"]),
                            parse_json_as_str(&network["name"]),
                        ) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                cfg.name, network
                            );
                            continue;
                        };
                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };
                        if let Ok(address) = parse_json_as_str(&network["contractAddress"]) {
                            parsed_network.set_contract(address);
                        }

                        if withdraw_enabled {
                            let withdraw_fee = parse_json_as_f64(&network["withdrawFee"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        }
                        if deposit_enabled {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["minConfirm"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        }
                    }
                }
            }
            Exchange::Bybit(cfg) => {
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let recv_window = "5000";
                        let timestamp = get_current_timestamp()?;
                        let signature = self.generate_signature(SignatureParams::Bybit {
                            query: "",
                            timestamp: &timestamp,
                            recv_window,
                        })?;

                        let headers = &[
                            ("X-BAPI-API-KEY", cfg.api_key.as_str()),
                            ("X-BAPI-RECV-WINDOW", recv_window),
                            ("X-BAPI-TIMESTAMP", &timestamp),
                            ("X-BAPI-SIGN", &signature),
                        ];
                        let response = cfg
                            .http_client
                            .get(endpoint, None, Some(headers), None)
                            .await?;
                        if response["retMsg"] == "success" {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                let rows = find_value_from_json_key(&data, &["result", "rows"])?;
                for item in rows.members() {
                    let Ok(asset_name) = parse_json_as_str(&item["coin"]) else {
                        continue;
                    };
                    if asset_name != coin {
                        continue;
                    }
                    for network in item["chains"].members() {
                        let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                            parse_json_as_bool(&network["chainWithdraw"]),
                            parse_json_as_bool(&network["chainDeposit"]),
                        ) else {
                            continue;
                        };

                        if !withdraw_enabled && !deposit_enabled {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&network["chain"]),
                            parse_json_as_str(&network["chainType"]),
                        ) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                network, cfg.name,
                            );
                            continue;
                        };

                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };

                        if let Ok(address) = parse_json_as_str(&network["contractAddress"]) {
                            parsed_network.set_contract(address);
                        }

                        if withdraw_enabled {
                            let withdraw_fee = parse_json_as_f64(&network["withdrawFee"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        };
                        if deposit_enabled {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["confirmation"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        };
                    }
                }
            }
            Exchange::Bitget(cfg) => {
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                None,
                                None,
                                Some(Duration::from_secs(BITGET_NETWORKS_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        if response["msg"] == "success" {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data["data"].members() {
                    let Ok(asset_name) = parse_json_as_str(&item["coin"]) else {
                        continue;
                    };
                    if asset_name != coin {
                        continue;
                    }
                    let networks = find_value_from_json_key(&item, &["chains"])?;
                    if !networks.is_array() {
                        return Err(format!(
                            "{} Invalid response: 'chains' is not an array",
                            cfg.name
                        )
                        .into());
                    }
                    for network in networks.members() {
                        let (Ok(withrawable), Ok(rechargeable)) = (
                            parse_json_as_bool(&network["withdrawable"]),
                            parse_json_as_bool(&network["rechargeable"]),
                        ) else {
                            continue;
                        };

                        if !withrawable && !rechargeable {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&network["chain"]),
                            parse_json_as_str(&network["chain"]),
                        ) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                cfg.name, network
                            );
                            continue;
                        };
                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };

                        if let Ok(address) = parse_json_as_str(&network["contractAddress"]) {
                            parsed_network.set_contract(address);
                        }

                        if withrawable {
                            let withdraw_fee = parse_json_as_f64(&network["withdrawFee"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        };
                        if rechargeable {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["depositConfirm"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        };

                        // fetched_networks.push(network);
                    }
                }
            }
            Exchange::Gate(cfg) => {
                let query = &[("currency", coin)];
                let headers = &[
                    ("Accept", "application/json"),
                    ("Content-Type", "application/json"),
                ];
                let networks = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;

                if networks.is_empty() {
                    return Err(format!("{} Invalid response: networks is empty", cfg.name).into());
                }

                for network in networks.members() {
                    let (Ok(withdraw_disabled), Ok(deposit_disabled)) = (
                        parse_json_as_bool(&network["is_withdraw_disabled"]),
                        parse_json_as_bool(&network["is_deposit_disabled"]),
                    ) else {
                        continue;
                    };

                    if deposit_disabled && withdraw_disabled {
                        continue;
                    }

                    let (Ok(name), Ok(full_name)) = (
                        parse_json_as_str(&network["chain"]),
                        parse_json_as_str(&network["name_en"]),
                    ) else {
                        error!(
                            "Не удалось распарсить название сети {} с биржи {}",
                            cfg.name, network
                        );
                        continue;
                    };
                    let Some(mut parsed_network) = Network::parse(name, full_name) else {
                        continue;
                    };

                    if let Ok(address) = parse_json_as_str(&network["contract_address"]) {
                        parsed_network.set_contract(address);
                    }

                    if !withdraw_disabled {
                        withdraw_networks.push(parsed_network.clone().make_withdraw_type(None));
                    };

                    if !deposit_disabled {
                        deposit_networks.push(parsed_network.make_deposit_type(None, None, None));
                    };
                }
            }
            Exchange::Kucoin(cfg) => {
                let data = match cache_data {
                    Some(data) => data,
                    None => {
                        let response = cfg.http_client.get(&endpoint, None, None, None).await?;
                        if response["code"] == "200000" {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };
                for item in data["data"].members() {
                    let Ok(asset_name) = parse_json_as_str(&item["currency"]) else {
                        continue;
                    };
                    if asset_name != coin {
                        continue;
                    }

                    let networks = find_value_from_json_key(&item, &["chains"])?;
                    for network in networks.members() {
                        let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                            parse_json_as_bool(&network["isWithdrawEnabled"]),
                            parse_json_as_bool(&network["isDepositEnabled"]),
                        ) else {
                            continue;
                        };

                        if !withdraw_enabled && !deposit_enabled {
                            continue;
                        }
                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&network["chainId"]),
                            parse_json_as_str(&network["chainName"]),
                        ) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                cfg.name, network
                            );
                            continue;
                        };
                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };
                        if let Ok(address) = parse_json_as_str(&network["contractAddress"]) {
                            parsed_network.set_contract(address);
                        }
                        if withdraw_enabled {
                            let withdraw_fee = parse_json_as_f64(&network["withdrawMinFee"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        };
                        if deposit_enabled {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["preConfirms"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        };
                    }
                }
            }
            Exchange::Mexc(cfg) => {
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let recv_window = "5000";
                        let timestamp = get_current_timestamp()?;
                        let query_string =
                            format!("recvWindow={}&timestamp={}", recv_window, timestamp);
                        let sign = self.generate_signature(SignatureParams::Mexc {
                            query: &query_string,
                            string_body: "",
                        })?;

                        let query = &[
                            ("recvWindow", recv_window),
                            ("timestamp", &timestamp),
                            ("signature", &sign),
                        ];

                        let headers = &[
                            ("X-MEXC-APIKEY", cfg.api_key.as_str()),
                            ("Content-Type", "application/json"),
                        ];
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                Some(query),
                                Some(headers),
                                Some(Duration::from_secs(MEXC_NETWORKS_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        if response.is_array() {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data.members() {
                    if item["coin"] != coin {
                        continue;
                    }
                    let networks = find_value_from_json_key(item, &["networkList"])?;
                    for network in networks.members() {
                        let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                            parse_json_as_bool(&network["withdrawEnable"]),
                            parse_json_as_bool(&network["depositEnable"]),
                        ) else {
                            continue;
                        };

                        if !withdraw_enabled && !deposit_enabled {
                            continue;
                        }
                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&network["netWork"]),
                            parse_json_as_str(&network["netWork"]),
                        ) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                cfg.name, network
                            );
                            continue;
                        };
                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };
                        if let Ok(address) = parse_json_as_str(&network["contract"]) {
                            parsed_network.set_contract(address);
                        }
                        if withdraw_enabled {
                            let withdraw_fee = parse_json_as_f64(&network["withdrawFee"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        };
                        if deposit_enabled {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["minConfirm"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        };
                    }
                }
            }
            Exchange::Huobi(cfg) => {
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let headers = &[("Content-Type", "application/json")];
                        let response = cfg
                            .http_client
                            .get(endpoint, None, Some(headers), None)
                            .await?;
                        if response["code"].to_string() == "200" {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data["data"].members() {
                    if item["currency"].to_string().to_uppercase() != coin {
                        continue;
                    };

                    for network in item["chains"].members() {
                        let deposit_enabled: bool = network["depositStatus"] == "allowed";
                        let withdraw_enabled: bool = network["withdrawStatus"] == "allowed";

                        let Ok(name) = parse_json_as_str(&network["chain"]) else {
                            error!(
                                "Не удалось распарсить название сети {} с биржи {}",
                                cfg.name, network
                            );
                            continue;
                        };
                        let full_name = match parse_json_as_str(&network["fullName"]) {
                            Ok(full_name) => full_name,
                            Err(_) => name.clone(),
                        };
                        let Some(mut parsed_network) = Network::parse(name, full_name) else {
                            continue;
                        };

                        if let Ok(address) = parse_json_as_str(&network["contractAddress"]) {
                            parsed_network.set_contract(address);
                        }
                        if withdraw_enabled {
                            let withdraw_fee =
                                parse_json_as_f64(&network["transactFeeWithdraw"]).ok();
                            withdraw_networks
                                .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                        };
                        if deposit_enabled {
                            let number_of_confirmation =
                                parse_json_as_u64(&network["numOfFastConfirmations"]).ok();
                            deposit_networks.push(parsed_network.make_deposit_type(
                                None,
                                None,
                                number_of_confirmation,
                            ));
                        };
                    }
                }
            }
            Exchange::Bitmart(cfg) => {
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let response = cfg.http_client.get(endpoint, None, None, None).await?;

                        if response["message"] == "OK" && !response["data"]["currencies"].is_empty()
                        {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data["data"]["currencies"].members() {
                    let currency_name = item["currency"]
                        .to_string()
                        .split('-')
                        .collect::<Vec<&str>>()[0]
                        .to_owned();

                    if currency_name.to_uppercase() != coin {
                        continue;
                    }

                    let (Ok(deposit_enabled), Ok(withdraw_enabled)) = (
                        parse_json_as_bool(&item["deposit_enabled"]),
                        parse_json_as_bool(&item["withdraw_enabled"]),
                    ) else {
                        continue;
                    };

                    let Ok(network_name) = parse_json_as_str(&item["network"]) else {
                        continue;
                    };

                    let Some(mut parsed_network) =
                        Network::parse(network_name.clone(), network_name)
                    else {
                        continue;
                    };

                    if let Ok(address) = parse_json_as_str(&item["contract_address"]) {
                        parsed_network.set_contract(address);
                    }
                    if withdraw_enabled {
                        let withdraw_fee = parse_json_as_f64(&item["withdraw_fee"]).ok();
                        withdraw_networks
                            .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                    };
                    if deposit_enabled {
                        deposit_networks.push(parsed_network.make_deposit_type(None, None, None));
                    };
                }
            }
            Exchange::Okx(cfg) => {
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let timestamp = get_timestamp_iso_8601_with_ms()?;
                        let sign_params = SignatureParams::Okx {
                            timestamp: &timestamp,
                            method: "GET",
                            request_path: &endpoint,
                            body: "",
                        };
                        let sign = self.generate_signature(sign_params)?;
                        let headers = &[
                            ("OK-ACCESS-KEY", cfg.api_key.as_str()),
                            ("OK-ACCESS-TIMESTAMP", timestamp.as_str()),
                            ("OK-ACCESS-SIGN", sign.as_str()),
                            ("OK-ACCESS-PASSPHRASE", &env::var("OKX_PASSPHRASE")?),
                        ];
                        let response = cfg
                            .http_client
                            .get(endpoint, None, Some(headers), None)
                            .await?;
                        if !response["data"].is_empty() {
                            cfg.cached_data.networks.set(response.clone()).await;
                        }
                        response
                    }
                };

                for item in data["data"].members() {
                    if item["ccy"] != coin {
                        continue;
                    }
                    let (Ok(deposit_allowed), Ok(withdraw_allowed)) = (
                        parse_json_as_bool(&item["canDep"]),
                        parse_json_as_bool(&item["canWd"]),
                    ) else {
                        continue;
                    };

                    let name =
                        item["chain"].to_string().split('-').collect::<Vec<&str>>()[1].to_string();

                    let Some(mut parsed_network) = Network::parse(name.clone(), name) else {
                        continue;
                    };

                    if let Ok(address) = parse_json_as_str(&item["ctAddr"]) {
                        parsed_network.set_contract(address);
                    }

                    if withdraw_allowed {
                        let withdraw_fee = parse_json_as_f64(&item["fee"]).ok();
                        withdraw_networks
                            .push(parsed_network.clone().make_withdraw_type(withdraw_fee));
                    };
                    if deposit_allowed {
                        let number_of_confirmation =
                            parse_json_as_u64(&item["minDepArrivalConfirm"]).ok();
                        deposit_networks.push(parsed_network.make_deposit_type(
                            None,
                            None,
                            number_of_confirmation,
                        ));
                    };
                }
            }
        }
        // if fetched_networks.len() == 0 {
        //     return Err(format!(
        //         "Error: Fetched networks is empty for {}",
        //         self.config().name
        //     )
        //     .into());
        // };
        Ok((withdraw_networks, deposit_networks))
    }
}
