use serde::Deserialize;
use serde_json;
use std::{collections::HashMap, fs, sync::OnceLock};

#[derive(Debug, Deserialize)]
struct Network {
    name: String,
    mappings: Vec<String>,
    tokens: Vec<String>,
}

static NETWORK_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_networks() -> HashMap<String, String> {
    let content = match fs::read_to_string("networks_mapping.json") {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    let networks: Vec<Network> = match serde_json::from_str(&content) {
        Ok(n) => n,
        Err(_) => return HashMap::new(),
    };

    let mut map = HashMap::new();
    for network in networks {
        for mapping in &network.mappings {
            map.insert(mapping.to_uppercase(), network.name.clone());
        }
        for token in &network.tokens {
            map.insert(token.to_uppercase(), network.name.clone());
        }
    }
    println!("RESULT {:#?}", map);
    map
}

pub fn get_map() -> &'static HashMap<String, String> {
    NETWORK_MAP.get_or_init(|| load_networks())
}
