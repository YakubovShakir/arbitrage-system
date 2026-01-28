use std::time::Duration;

use async_trait::async_trait;

use crate::{
    config::parameters::{
        BINANCE_NETWORKS_HTTP_TIMEOUT_SECONDS, BITGET_NETWORKS_HTTP_TIMEOUT_SECONDS, ERROR_CODE,
        MEXC_NETWORKS_HTTP_TIMEOUT_SECONDS, RESET_CODE,
    },
    core::{
        traits::exchange_service::NetworkService,
        types::{API, Network, exchanges::Exchange, signature_params::SignatureParams},
        utils::{
            get_current_timestamp,
            json_utils::{
                find_value_from_json_key, parse_json_as_bool, parse_json_as_f64, parse_json_as_str,
            },
        },
    },
};

#[async_trait]
impl NetworkService for Exchange {
    async fn networks(&self, coin: &str) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        let Some(endpoint) = API::GetNetworks.endpoint(self) else {
            return Err(format!(
                "Error: GetNetworks endpoint is not set for the exchange {}",
                self.config().name
            )
            .into());
        };
        let mut fetched_networks: Vec<Network> = Vec::new();
        let cache_data = self.config().cached_data.networks.get().await;

        match self {
            Exchange::Binance(cfg) => {
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
                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                Some(query),
                                Some(headers),
                                Some(Duration::from_secs(BINANCE_NETWORKS_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        cfg.cached_data.networks.set(response.clone()).await;
                        response
                    }
                };

                for item in data.members() {
                    if item["coin"] != coin {
                        continue;
                    }

                    let deposit_enabled = parse_json_as_bool(&item["depositAllEnable"])?;
                    let withdraw_enabled = parse_json_as_bool(&item["withdrawAllEnable"])?;
                    if !deposit_enabled || !withdraw_enabled {
                        break;
                    }

                    let networks = &item["networkList"];
                    for fetched_network in networks.members() {
                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&fetched_network["network"]),
                            parse_json_as_str(&fetched_network["name"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, fetched_network
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&fetched_network["withdrawFee"])
                        {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&fetched_network["contractAddress"])
                        {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
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
                    let networks = find_value_from_json_key(&item, &["chains"])?;
                    for fetched_network in networks.members() {
                        if fetched_network["chainDeposit"] != "1"
                            || fetched_network["chainWithdraw"] != "1"
                        {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&fetched_network["chain"]),
                            parse_json_as_str(&fetched_network["chainType"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, fetched_network
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&fetched_network["withdrawFee"])
                        {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&fetched_network["contractAddress"])
                        {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
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

                    let chains = find_value_from_json_key(&item, &["chains"])?;
                    if !chains.is_array() {
                        return Err(format!(
                            "{} Invalid response: 'chains' is not an array",
                            cfg.name
                        )
                        .into());
                    }
                    for chain in chains.members() {
                        if let (Ok(withrawable), Ok(rechargeable)) = (
                            parse_json_as_bool(&chain["withdrawable"]),
                            parse_json_as_bool(&chain["rechargeable"]),
                        ) {
                            if !withrawable || !rechargeable {
                                continue;
                            }
                        } else {
                            continue;
                        };

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&chain["chain"]),
                            parse_json_as_str(&chain["chain"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, chain
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&chain["withdrawFee"]) {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&chain["contractAddress"]) {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
                    }
                }
            }
            Exchange::Gate(cfg) => {
                let query = &[("currency", coin)];
                let headers = &[
                    ("Accept", "application/json"),
                    ("Content-Type", "application/json"),
                ];
                let chains = cfg
                    .http_client
                    .get(endpoint, Some(query), Some(headers), None)
                    .await?;

                if chains.is_empty() {
                    return Err(
                        format!("{} Invalid response: 'chains' is not an array", cfg.name).into(),
                    );
                }

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

                    let (Ok(name), Ok(full_name)) = (
                        parse_json_as_str(&chain["chain"]),
                        parse_json_as_str(&chain["name_en"]),
                    ) else {
                        println!(
                            "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                            cfg.name, chain
                        );
                        continue;
                    };
                    let Some(mut network) = Network::parse(name, full_name, coin.to_owned()) else {
                        continue;
                    };
                    if let Ok(address) = parse_json_as_str(&chain["contract_address"]) {
                        network.set_contract(address);
                    }

                    fetched_networks.push(network);
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
                    let chains = find_value_from_json_key(&item, &["chains"])?;
                    for chain in chains.members() {
                        if let (Ok(withdraw_enabled), Ok(deposit_enabled)) = (
                            parse_json_as_bool(&chain["isWithdrawEnabled"]),
                            parse_json_as_bool(&chain["isDepositEnabled"]),
                        ) {
                            if !withdraw_enabled || !deposit_enabled {
                                continue;
                            }
                        } else {
                            continue;
                        };

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&chain["chainId"]),
                            parse_json_as_str(&chain["chainName"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, chain
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&chain["withdrawMinFee"]) {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&chain["contractAddress"]) {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
                    }
                }
            }
            Exchange::Mexc(cfg) => {
                let cache_data = cfg.cached_data.networks.get().await;

                let recv_window = "5000";
                let timestamp = get_current_timestamp()?;
                let query_string = format!("recvWindow={}&timestamp={}", recv_window, timestamp);
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

                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
                        let response = cfg
                            .http_client
                            .get(
                                endpoint,
                                Some(query),
                                Some(headers),
                                Some(Duration::from_secs(MEXC_NETWORKS_HTTP_TIMEOUT_SECONDS)),
                            )
                            .await?;
                        cfg.cached_data.networks.set(response.clone()).await;
                        response
                    }
                };

                for item in data.members() {
                    if item["coin"] != coin {
                        continue;
                    }

                    let networks = find_value_from_json_key(item, &["networkList"])?;
                    for chain in networks.members() {
                        let withdraw_enabled = parse_json_as_bool(&chain["withdrawEnable"])?;
                        if !withdraw_enabled {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&chain["netWork"]),
                            parse_json_as_str(&chain["netWork"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, chain
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&chain["withdrawFee"]) {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&chain["contract"]) {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
                    }
                }
            }
            Exchange::Huobi(cfg) => {
                let headers = &[("Content-Type", "application/json")];

                let data = match cache_data {
                    Some(cached) => cached,
                    None => {
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
                    let chains = &item["chains"];
                    for chain in chains.members() {
                        if chain["depositStatus"] != "allowed"
                            || chain["withdrawStatus"] != "allowed"
                        {
                            continue;
                        }

                        let (Ok(name), Ok(full_name)) = (
                            parse_json_as_str(&chain["chain"]),
                            parse_json_as_str(&chain["fullName"]),
                        ) else {
                            println!(
                                "{ERROR_CODE} Не удалось распарсить название сети {} с биржи {} {RESET_CODE}",
                                cfg.name, chain
                            );
                            continue;
                        };
                        let Some(mut network) = Network::parse(name, full_name, coin.to_owned())
                        else {
                            continue;
                        };
                        if let Ok(withdraw_fee) = parse_json_as_f64(&chain["transactFeeWithdraw"]) {
                            network.set_withdraw_fee(withdraw_fee);
                        }
                        if let Ok(address) = parse_json_as_str(&chain["contractAddress"]) {
                            network.set_contract(address);
                        }

                        fetched_networks.push(network);
                    }
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
        Ok(fetched_networks)
    }
}
