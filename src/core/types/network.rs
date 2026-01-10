use std::{collections::HashMap, sync::OnceLock};

use crate::config::parameters::{ERROR_CODE, RESET_CODE};

static NETWORK_MAP: OnceLock<HashMap<&'static str, NetworkType>> = OnceLock::new();

fn get_map() -> &'static HashMap<&'static str, NetworkType> {
    NETWORK_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        // Cyber
        const CYBER_MAPPING: [&str; 4] =
            ["CYBER", "CYBERCONNECT", "CYBER NETWORK", "CYBER PROTOCOL"];
        for mapped in CYBER_MAPPING {
            m.insert(mapped, NetworkType::Cyber);
        }

        // Eclipse
        const ECLIPSE_MAPPING: [&str; 3] = ["ES", "ECLIPSE", "ECLIPSELAYER"];
        for mapped in ECLIPSE_MAPPING {
            m.insert(mapped, NetworkType::Eclipse);
        }
        // Gravity
        const GRAVITY_MAPPING: [&str; 5] = [
            "G",
            "GRAVITY",
            "GRAVITY ALPHA",
            "GRAVITY PROTOCOL",
            "GRAVITYCHAIN",
        ];
        for mapped in GRAVITY_MAPPING {
            m.insert(mapped, NetworkType::Gravity);
        }
        // OM Network
        const OM_MAPPING: [&str; 3] = ["OM", "OMNETWORK", "MANTRANETWORK"];
        for mapped in OM_MAPPING {
            m.insert(mapped, NetworkType::Om);
        }

        // Viction (ранее TomoChain)
        const VICTION_MAPPING: [&str; 4] = ["VIC", "VICTION", "TOMOTOMAINNET", "TOMOTOMAIN"];
        for mapped in VICTION_MAPPING {
            m.insert(mapped, NetworkType::Viction);
        }

        // Verge (XVG)
        const VERGE_MAPPING: [&str; 3] = ["XVG", "VERGE", "VERGECURRENCY"];
        for mapped in VERGE_MAPPING {
            m.insert(mapped, NetworkType::Verge);
        }

        // Initia
        const INITIA_MAPPING: [&str; 3] = ["INITIA", "INITIA NETWORK", "INITIA PROTOCOL"];
        for mapped in INITIA_MAPPING {
            m.insert(mapped, NetworkType::Initia);
        }
        // Babylon
        const BABYLON_MAPPING: [&str; 3] = ["BABY", "BABYLON", "BABYLONCHAIN"];
        for mapped in BABYLON_MAPPING {
            m.insert(mapped, NetworkType::Babylon);
        }
        // QuarkChain
        const QUARKCHAIN_MAPPING: [&str; 3] = ["QKC", "QUARKCHAIN", "QUARK"];
        for mapped in QUARKCHAIN_MAPPING {
            m.insert(mapped, NetworkType::QuarkChain);
        }

        // BitLayer
        const BITLAYER_MAPPING: [&str; 4] =
            ["BTRBTC", "BITLAYER", "BITLAYERCHAIN", "BITLAYERNETWORK"];
        for mapped in BITLAYER_MAPPING {
            m.insert(mapped, NetworkType::BitLayer);
        }

        // Asset Hub Kusama
        const ASSET_HUB_KUSAMA_MAPPING: [&str; 3] =
            ["ASSETHUBKUSAMA", "KUSAMAASSETHUB", "KUSAMAHUB"];
        for mapped in ASSET_HUB_KUSAMA_MAPPING {
            m.insert(mapped, NetworkType::AssetHubKusama);
        }

        // Terra 2.0 (LUNA2)
        const TERRA_MAPPING: [&str; 4] = ["LUNA2", "TERRA2", "LUNA", "TERRA"];
        for mapped in TERRA_MAPPING {
            m.insert(mapped, NetworkType::Terra);
        }

        // Terra Classic
        const TERRA_CLASSIC_MAPPING: [&str; 6] = [
            "LUNC",
            "TERRA CLASSIC",
            "LUNA CLASSIC",
            "TERRA",
            "LUNA",
            "TERRALUNA",
        ];
        for mapped in TERRA_CLASSIC_MAPPING {
            m.insert(mapped, NetworkType::TerraClassic);
        }

        // 0G Chain
        const ZEROG_MAPPING: [&str; 1] = ["0G"];
        for mapped in ZEROG_MAPPING {
            m.insert(mapped, NetworkType::ZeroG);
        }
        // Bittensor
        const BITTENSOR_MAPPING: [&str; 4] = [
            "TAO",
            "BITTENSOR",
            "BITTENSOR NETWORK",
            "BITTENSOR PROTOCOL",
        ];
        for mapped in BITTENSOR_MAPPING {
            m.insert(mapped, NetworkType::Bittensor);
        }

        // Bera
        const BERA_MAPPING: [&str; 2] = ["BERA", "BERA CHAIN"];
        for mapped in BERA_MAPPING {
            m.insert(mapped, NetworkType::Bera);
        }

        // Celo
        const CELO_MAPPING: [&str; 3] = ["CELO", "CELO NETWORK", "CELO BLOCKCHAIN"];
        for mapped in CELO_MAPPING {
            m.insert(mapped, NetworkType::Celo);
        }
        // Endurance
        const ENDURANCE_MAPPING: [&str; 2] = ["ENDURANCE", "ENDURANCE CHAIN"];
        for mapped in ENDURANCE_MAPPING {
            m.insert(mapped, NetworkType::Endurance);
        }

        // Movement Network
        const MOVEMENT_MAPPING: [&str; 3] = ["MOVE", "MOVEMENT", "MOVEMENT NETWORK"];
        for mapped in MOVEMENT_MAPPING {
            m.insert(mapped, NetworkType::Movement);
        }

        // Bitcoin
        const BITCOIN_MAPPING: [&str; 4] = ["BTC", "BITCOIN", "BTCBRC", "BRC20"];
        for mapped in BITCOIN_MAPPING {
            m.insert(mapped, NetworkType::Bitcoin);
        }

        // Ethereum
        const ETHEREUM_MAPPING: [&str; 3] = ["ETHEREUM", "ETH", "ERC20"];
        for mapped in ETHEREUM_MAPPING {
            m.insert(mapped, NetworkType::Ethereum);
        }

        // BinanceSmartChain
        const BSC_MAPPING: [&str; 3] = ["BEP20", "BSC", "BINANCE SMART CHAIN"];
        for mapped in BSC_MAPPING {
            m.insert(mapped, NetworkType::BinanceSmartChain);
        }

        // Solana
        const SOLANA_MAPPING: [&str; 2] = ["SOLANA", "SOL"];
        for mapped in SOLANA_MAPPING {
            m.insert(mapped, NetworkType::Solana);
        }

        // Sui
        const SUI_MAPPING: [&str; 1] = ["SUI"];
        for mapped in SUI_MAPPING {
            m.insert(mapped, NetworkType::Sui);
        }

        // Stacks
        const STX_MAPPING: [&str; 2] = ["STX", "STACKS"];
        for mapped in STX_MAPPING {
            m.insert(mapped, NetworkType::Stacks);
        }

        // Mina
        const MINA_MAPPING: [&str; 2] = ["MINA", "MINA PROTOCOL"];
        for mapped in MINA_MAPPING {
            m.insert(mapped, NetworkType::Mina);
        }

        // Vanar
        const VANAR_MAPPING: [&str; 3] = ["VANRY", "VANAR", "VANAR CHAIN"];
        for mapped in VANAR_MAPPING {
            m.insert(mapped, NetworkType::Vanar);
        }

        // MultiversX (бывший Elrond)
        const MULTIVERSX_MAPPING: [&str; 3] = ["EGLD", "MULTIVERSX", "ELROND"];
        for mapped in MULTIVERSX_MAPPING {
            m.insert(mapped, NetworkType::MultiversX);
        }

        // RSK (Rootstock)
        const ROOTSTOCK_MAPPING: [&str; 3] = ["SOVRBTC", "RSK", "ROOTSTOCK"];
        for mapped in ROOTSTOCK_MAPPING {
            m.insert(mapped, NetworkType::Rootstock);
        }

        // Scroll
        const SCROLL_MAPPING: [&str; 2] = ["SCROLLETH", "SCROLL"];
        for mapped in SCROLL_MAPPING {
            m.insert(mapped, NetworkType::Scroll);
        }

        // The Open Network
        const TON_MAPPING: [&str; 3] = ["TON", "THE OPEN NETWORK", "TONCOIN"];
        for mapped in TON_MAPPING {
            m.insert(mapped, NetworkType::Ton);
        }

        // ICON Network
        const ICX_MAPPING: [&str; 2] = ["ICX", "ICON"];
        for mapped in ICX_MAPPING {
            m.insert(mapped, NetworkType::Icon);
        }

        // Polygon
        const POLYGON_MAPPING: [&str; 2] = ["MATIC", "POLYGON"];
        for mapped in POLYGON_MAPPING {
            m.insert(mapped, NetworkType::Polygon);
        }

        // Harmony
        const HARMONY_MAPPING: [&str; 2] = ["ONE", "HARMONY"];
        for mapped in HARMONY_MAPPING {
            m.insert(mapped, NetworkType::Harmony);
        }

        // zkLink Nova
        const ZKLINK_MAPPING: [&str; 3] = ["ZKLETH", "ZKLINK", "ZKLINKNOVA"];
        for mapped in ZKLINK_MAPPING {
            m.insert(mapped, NetworkType::ZkLinkNova);
        }

        // Base
        const BASE_MAPPING: [&str; 2] = ["BASE", "BASE MAINNET"];
        for mapped in BASE_MAPPING {
            m.insert(mapped, NetworkType::Base);
        }

        // Hemi
        const HEMI_MAPPING: [&str; 2] = ["HEMI", "HEMI NETWORK"];
        for mapped in HEMI_MAPPING {
            m.insert(mapped, NetworkType::Hemi);
        }

        // GUNZ Network
        const GUNZ_MAPPING: [&str; 3] = ["GUN", "GUNZ", "GUNZ NETWORK"];
        for mapped in GUNZ_MAPPING {
            m.insert(mapped, NetworkType::GunzNetwork);
        }

        // Avalanche C-Chain
        const AVALANCHE_MAPPING: [&str; 3] = ["AVAXC", "AVAX", "AVALANCHE"];
        for mapped in AVALANCHE_MAPPING {
            m.insert(mapped, NetworkType::Avalanche);
        }

        // Arbitrum
        const ARBITRUM_MAPPING: [&str; 5] =
            ["ARBITRUM", "ARBITRUMONE", "ARB", "ARBEVM", "ARBITRUMEVO"];
        for mapped in ARBITRUM_MAPPING {
            m.insert(mapped, NetworkType::Arbitrum);
        }

        // World Chain
        const WORLD_CHAIN_MAPPING: [&str; 3] = ["WLD", "WORLD", "WORLDCHAIN"];
        for mapped in WORLD_CHAIN_MAPPING {
            m.insert(mapped, NetworkType::WorldChain);
        }

        // Optimism
        const OPTIMISM_MAPPING: [&str; 4] = ["OPETH", "OPTIMISM", "OP", "OPTIMISTIC ETHEREUM"];
        for mapped in OPTIMISM_MAPPING {
            m.insert(mapped, NetworkType::Optimism);
        }

        // Flux
        const FLUX_MAPPING: [&str; 2] = ["FLUX", "FLUX NETWORK"];
        for mapped in FLUX_MAPPING {
            m.insert(mapped, NetworkType::Flux);
        }

        // Celestia
        const CELESTIA_MAPPING: [&str; 3] = ["TIA", "CELESTIA", "CELESTIA NETWORK"];
        for mapped in CELESTIA_MAPPING {
            m.insert(mapped, NetworkType::Celestia);
        }

        // Metis
        const METIS_MAPPING: [&str; 3] = ["METIS TOKEN", "METIS", "METIS NETWORK"];
        for mapped in METIS_MAPPING {
            m.insert(mapped, NetworkType::Metis);
        }

        // Linea
        const LINEA_MAPPING: [&str; 3] = ["LINEA", "LINEA NETWORK", "CONSENSYS LINEA"];
        for mapped in LINEA_MAPPING {
            m.insert(mapped, NetworkType::Linea);
        }

        // IOST
        const IOST_MAPPING: [&str; 3] = ["IOST", "IOST NETWORK", "INTERNET OF SERVICES"];
        for mapped in IOST_MAPPING {
            m.insert(mapped, NetworkType::Iost);
        }

        // COTI
        const COTI_MAPPING: [&str; 3] = ["COTI", "COTI NETWORK", "CURRENCY OF INTERNET"];
        for mapped in COTI_MAPPING {
            m.insert(mapped, NetworkType::Coti);
        }

        // NEO N3
        const NEO_MAPPING: [&str; 4] = ["NEO3", "NEO", "NEON3", "NEO BLOCKCHAIN"];
        for mapped in NEO_MAPPING {
            m.insert(mapped, NetworkType::Neo);
        }
        // Moonbeam (GLMR - токен Moonbeam)
        const MOONBEAM_MAPPING: [&str; 3] = ["GLMR", "MOONBEAM", "MOON"];
        for mapped in MOONBEAM_MAPPING {
            m.insert(mapped, NetworkType::Moonbeam);
        }

        // Chiliz Chain
        const CHILIZ_MAPPING: [&str; 5] = ["CAP20", "CHZ2", "CHZ", "CHILIZ", "CHILIZ CHAIN"];
        for mapped in CHILIZ_MAPPING {
            m.insert(mapped, NetworkType::Chiliz);
        }

        // Bifrost
        const BIFROST_MAPPING: [&str; 3] = ["BNCDOT", "BNC", "BIFROST"];
        for mapped in BIFROST_MAPPING {
            m.insert(mapped, NetworkType::Bifrost);
        }

        // dYdX
        const DYDX_MAPPING: [&str; 2] = ["DYDX", "DYDX CHAIN"];
        for mapped in DYDX_MAPPING {
            m.insert(mapped, NetworkType::Dydx);
        }

        // Nero
        const NERO_MAPPING: [&str; 1] = ["NERO"];
        for mapped in NERO_MAPPING {
            m.insert(mapped, NetworkType::Nero);
        }

        // Decred
        const DECRED_MAPPING: [&str; 2] = ["DCR", "DECRED"];
        for mapped in DECRED_MAPPING {
            m.insert(mapped, NetworkType::Decred);
        }

        // Tron
        const TRON_MAPPING: [&str; 4] = ["TRX", "TRON", "TRC20", "TRON NETWORK"];
        for mapped in TRON_MAPPING {
            m.insert(mapped, NetworkType::Tron);
        }

        // Ontology
        const ONTOLOGY_MAPPING: [&str; 2] = ["ONT", "ONTOLOGY"];
        for mapped in ONTOLOGY_MAPPING {
            m.insert(mapped, NetworkType::Ontology);
        }

        // Dash
        const DASH_MAPPING: [&str; 2] = ["DASH", "DASHCOIN"];
        for mapped in DASH_MAPPING {
            m.insert(mapped, NetworkType::Dash);
        }

        // Mango
        const MANGO_MAPPING: [&str; 3] = ["MANGO", "MGO", "MANGONET"];
        for mapped in MANGO_MAPPING {
            m.insert(mapped, NetworkType::Mango);
        }

        // Story
        const STORY_MAPPING: [&str; 2] = ["STORY", "STORYNET"];
        for mapped in STORY_MAPPING {
            m.insert(mapped, NetworkType::Story);
        }

        m
    })
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub full_name: String,
    pub coin_name: String,
    pub withdraw_fee: Option<f64>,
    pub contract_address: Option<String>,
    pub memo: Option<String>,
    pub deposit_address: Option<String>,
}

enum NetworkType {
    Bitcoin,
    Ethereum,
    BinanceSmartChain,
    Solana,
    Sui,
    Stacks,
    Mina,
    Vanar,
    MultiversX,
    Rootstock,
    Scroll,
    Ton,
    Icon,
    Polygon,
    Harmony,
    ZkLinkNova,
    Base,
    Hemi,
    GunzNetwork,
    Avalanche,
    Arbitrum,
    WorldChain,
    Optimism,
    Flux,
    Celestia,
    Metis,
    Linea,
    Iost,
    Coti,
    Neo,
    Moonbeam,
    Chiliz,
    Bifrost,
    Dydx,
    Nero,
    Decred,
    Tron,
    Ontology,
    Dash,
    Mango,
    Story,
    Cyber,
    Eclipse,
    Gravity,
    Om,
    Viction,
    Verge,
    Initia,
    Babylon,
    QuarkChain,
    BitLayer,
    AssetHubKusama,
    Terra,
    TerraClassic,
    ZeroG,
    Bittensor,
    Bera,
    Celo,
    Endurance,
    Movement,
}

#[derive(Debug, Clone)]
pub enum Network {
    Bitcoin(NetworkConfig),
    Ethereum(NetworkConfig),
    BinanceSmartChain(NetworkConfig),
    Solana(NetworkConfig),
    Sui(NetworkConfig),
    Stacks(NetworkConfig),
    Mina(NetworkConfig),
    Vanar(NetworkConfig),
    MultiversX(NetworkConfig),
    Rootstock(NetworkConfig),
    Scroll(NetworkConfig),
    Ton(NetworkConfig),
    Icon(NetworkConfig),
    Polygon(NetworkConfig),
    Harmony(NetworkConfig),
    ZkLinkNova(NetworkConfig),
    Base(NetworkConfig),
    Hemi(NetworkConfig),
    GunzNetwork(NetworkConfig),
    Avalanche(NetworkConfig),
    Arbitrum(NetworkConfig),
    WorldChain(NetworkConfig),
    Optimism(NetworkConfig),
    Flux(NetworkConfig),
    Celestia(NetworkConfig),
    Metis(NetworkConfig),
    Linea(NetworkConfig),
    Iost(NetworkConfig),
    Coti(NetworkConfig),
    Neo(NetworkConfig),
    Moonbeam(NetworkConfig),
    Chiliz(NetworkConfig),
    Bifrost(NetworkConfig),
    Dydx(NetworkConfig),
    Nero(NetworkConfig),
    Decred(NetworkConfig),
    Tron(NetworkConfig),
    Ontology(NetworkConfig),
    Dash(NetworkConfig),
    Mango(NetworkConfig),
    Story(NetworkConfig),
    Cyber(NetworkConfig),
    Eclipse(NetworkConfig),
    Gravity(NetworkConfig),
    Om(NetworkConfig),
    Viction(NetworkConfig),
    Verge(NetworkConfig),
    Initia(NetworkConfig),
    Babylon(NetworkConfig),
    QuarkChain(NetworkConfig),
    BitLayer(NetworkConfig),
    AssetHubKusama(NetworkConfig),
    Terra(NetworkConfig),
    TerraClassic(NetworkConfig),
    ZeroG(NetworkConfig),
    Bittensor(NetworkConfig),
    Bera(NetworkConfig),
    Celo(NetworkConfig),
    Endurance(NetworkConfig),
    Movement(NetworkConfig),
}

impl Network {
    pub fn parse(name: String, full_name: String, coin_name: String) -> Option<Self> {
        let config = NetworkConfig {
            name,
            full_name,
            coin_name,
            withdraw_fee: None,
            contract_address: None,
            memo: None,
            deposit_address: None,
        };
        let mapped = Self::get_network_type(&config.name)
            .or_else(|| Network::get_network_type(&config.full_name));
        match mapped {
            Some(network_type) => match network_type {
                NetworkType::Bitcoin => Some(Self::Bitcoin(config)),
                NetworkType::Ethereum => Some(Self::Ethereum(config)),
                NetworkType::BinanceSmartChain => Some(Self::BinanceSmartChain(config)),
                NetworkType::Solana => Some(Self::Solana(config)),
                NetworkType::Sui => Some(Self::Sui(config)),
                NetworkType::Stacks => Some(Self::Stacks(config)),
                NetworkType::Mina => Some(Self::Mina(config)),
                NetworkType::Vanar => Some(Self::Vanar(config)),
                NetworkType::MultiversX => Some(Self::MultiversX(config)),
                NetworkType::Rootstock => Some(Self::Rootstock(config)),
                NetworkType::Scroll => Some(Self::Scroll(config)),
                NetworkType::Ton => Some(Self::Ton(config)),
                NetworkType::Icon => Some(Self::Icon(config)),
                NetworkType::Polygon => Some(Self::Polygon(config)),
                NetworkType::Harmony => Some(Self::Harmony(config)),
                NetworkType::ZkLinkNova => Some(Self::ZkLinkNova(config)),
                NetworkType::Base => Some(Self::Base(config)),
                NetworkType::Hemi => Some(Self::Hemi(config)),
                NetworkType::GunzNetwork => Some(Self::GunzNetwork(config)),
                NetworkType::Avalanche => Some(Self::Avalanche(config)),
                NetworkType::Arbitrum => Some(Self::Arbitrum(config)),
                NetworkType::WorldChain => Some(Self::WorldChain(config)),
                NetworkType::Optimism => Some(Self::Optimism(config)),
                NetworkType::Flux => Some(Self::Flux(config)),
                NetworkType::Celestia => Some(Self::Celestia(config)),
                NetworkType::Metis => Some(Self::Metis(config)),
                NetworkType::Linea => Some(Self::Linea(config)),
                NetworkType::Iost => Some(Self::Iost(config)),
                NetworkType::Coti => Some(Self::Coti(config)),
                NetworkType::Neo => Some(Self::Neo(config)),
                NetworkType::Moonbeam => Some(Self::Moonbeam(config)),
                NetworkType::Chiliz => Some(Self::Chiliz(config)),
                NetworkType::Bifrost => Some(Self::Bifrost(config)),
                NetworkType::Dydx => Some(Self::Dydx(config)),
                NetworkType::Nero => Some(Self::Nero(config)),
                NetworkType::Decred => Some(Self::Decred(config)),
                NetworkType::Tron => Some(Self::Tron(config)),
                NetworkType::Ontology => Some(Self::Ontology(config)),
                NetworkType::Dash => Some(Self::Dash(config)),
                NetworkType::Mango => Some(Self::Mango(config)),
                NetworkType::Story => Some(Self::Story(config)),
                NetworkType::Cyber => Some(Self::Cyber(config)),
                NetworkType::Eclipse => Some(Self::Eclipse(config)),
                NetworkType::Gravity => Some(Self::Gravity(config)),
                NetworkType::Om => Some(Self::Om(config)),
                NetworkType::Viction => Some(Self::Viction(config)),
                NetworkType::Verge => Some(Self::Verge(config)),
                NetworkType::Initia => Some(Self::Initia(config)),
                NetworkType::Babylon => Some(Self::Babylon(config)),
                NetworkType::QuarkChain => Some(Self::QuarkChain(config)),
                NetworkType::BitLayer => Some(Self::BitLayer(config)),
                NetworkType::AssetHubKusama => Some(Self::AssetHubKusama(config)),
                NetworkType::Terra => Some(Self::Terra(config)),
                NetworkType::TerraClassic => Some(Self::TerraClassic(config)),
                NetworkType::ZeroG => Some(Self::ZeroG(config)),
                NetworkType::Bittensor => Some(Self::Bittensor(config)),
                NetworkType::Bera => Some(Self::Bera(config)),
                NetworkType::Celo => Some(Self::Celo(config)),
                NetworkType::Endurance => Some(Self::Endurance(config)),
                NetworkType::Movement => Some(Self::Movement(config)),
            },

            None => {
                println!(
                    "{ERROR_CODE}[ERROR] Не удалось распознать сеть name: {} full_name: {}{RESET_CODE}",
                    config.name, config.full_name
                );
                None
            }
        }
    }
    fn get_network_type(name: &str) -> Option<&NetworkType> {
        get_map().get(&*name.to_uppercase())
    }
    pub fn set_deposit_addresss(&mut self, address: String) -> &Self {
        self.mut_config().deposit_address = Some(address);
        self
    }
    pub fn set_memo(&mut self, memo: String) -> &Self {
        self.mut_config().memo = Some(memo);
        self
    }
    pub fn set_withdraw_fee(&mut self, fee: f64) -> &Self {
        self.mut_config().withdraw_fee = Some(fee);
        self
    }
    pub fn set_contract(&mut self, address: String) -> &Self {
        self.mut_config().contract_address = Some(address);
        self
    }
    fn mut_config(&mut self) -> &mut NetworkConfig {
        match self {
            Self::Bitcoin(cfg) => cfg,
            Self::Ethereum(cfg) => cfg,
            Self::BinanceSmartChain(cfg) => cfg,
            Self::Solana(cfg) => cfg,
            Self::Sui(cfg) => cfg,
            Self::Stacks(cfg) => cfg,
            Self::Mina(cfg) => cfg,
            Self::Vanar(cfg) => cfg,
            Self::MultiversX(cfg) => cfg,
            Self::Rootstock(cfg) => cfg,
            Self::Scroll(cfg) => cfg,
            Self::Ton(cfg) => cfg,
            Self::Icon(cfg) => cfg,
            Self::Polygon(cfg) => cfg,
            Self::Harmony(cfg) => cfg,
            Self::ZkLinkNova(cfg) => cfg,
            Self::Base(cfg) => cfg,
            Self::Hemi(cfg) => cfg,
            Self::GunzNetwork(cfg) => cfg,
            Self::Avalanche(cfg) => cfg,
            Self::Arbitrum(cfg) => cfg,
            Self::WorldChain(cfg) => cfg,
            Self::Optimism(cfg) => cfg,
            Self::Flux(cfg) => cfg,
            Self::Celestia(cfg) => cfg,
            Self::Metis(cfg) => cfg,
            Self::Linea(cfg) => cfg,
            Self::Iost(cfg) => cfg,
            Self::Coti(cfg) => cfg,
            Self::Neo(cfg) => cfg,
            Self::Moonbeam(cfg) => cfg,
            Self::Chiliz(cfg) => cfg,
            Self::Bifrost(cfg) => cfg,
            Self::Dydx(cfg) => cfg,
            Self::Nero(cfg) => cfg,
            Self::Decred(cfg) => cfg,
            Self::Tron(cfg) => cfg,
            Self::Ontology(cfg) => cfg,
            Self::Dash(cfg) => cfg,
            Self::Mango(cfg) => cfg,
            Self::Story(cfg) => cfg,
            Self::Cyber(cfg) => cfg,
            Self::Eclipse(cfg) => cfg,
            Self::Gravity(cfg) => cfg,
            Self::Om(cfg) => cfg,
            Self::Viction(cfg) => cfg,
            Self::Verge(cfg) => cfg,
            Self::Initia(cfg) => cfg,
            Self::Babylon(cfg) => cfg,
            Self::QuarkChain(cfg) => cfg,
            Self::BitLayer(cfg) => cfg,
            Self::AssetHubKusama(cfg) => cfg,
            Self::Terra(cfg) => cfg,
            Self::TerraClassic(cfg) => cfg,
            Self::ZeroG(cfg) => cfg,
            Self::Bittensor(cfg) => cfg,
            Self::Bera(cfg) => cfg,
            Self::Celo(cfg) => cfg,
            Self::Endurance(cfg) => cfg,
            Self::Movement(cfg) => cfg,
        }
    }

    pub fn create_test() -> Network {
        Network::Bitcoin(NetworkConfig {
            name: "TestNet".to_string(),
            full_name: "TestNetwork".to_string(),
            coin_name: "TestNet".to_string(),
            withdraw_fee: None,
            contract_address: None,
            memo: None,
            deposit_address: None,
        })
    }
}
