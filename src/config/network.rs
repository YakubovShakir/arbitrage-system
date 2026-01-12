use std::{collections::HashMap, sync::OnceLock};

use crate::core::types::network::NetworkType;

static NETWORK_MAP: OnceLock<HashMap<&'static str, NetworkType>> = OnceLock::new();

pub fn get_map() -> &'static HashMap<&'static str, NetworkType> {
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

        // Kusama
        const KUSAMA_MAPPING: [&str; 4] = ["KSMSM", "KUSAMA", "KSM", "KUSAMANETWORK"];
        for mapped in KUSAMA_MAPPING {
            m.insert(mapped, NetworkType::Kusama);
        }

        // Asset Hub Kusama
        const ASSET_HUB_KUSAMA_MAPPING: [&str; 3] =
            ["ASSET HUB KUSAMA", "KUSAMA ASSET HUB", "KUSAMA HUB"];
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
        // Plume
        const PLUME_MAPPING: [&str; 3] = ["PLUME", "PLUMENETWORK", "PLUME BLOCKCHAIN"];
        for mapped in PLUME_MAPPING {
            m.insert(mapped, NetworkType::Plume);
        }

        // XRP Ledger
        const XRP_MAPPING: [&str; 3] = ["XRP", "XRPL", "RIPPLE"];
        for mapped in XRP_MAPPING {
            m.insert(mapped, NetworkType::Xrp);
        }

        // Harmony
        const HARMONY_MAPPING: [&str; 2] = ["ONE", "HARMONY"];
        for mapped in HARMONY_MAPPING {
            m.insert(mapped, NetworkType::Harmony);
        }
        // zkSync Era
        const ZKSYNC_MAPPING: [&str; 2] = ["ZKSYNCERA", "ZKSYNC"];
        for mapped in ZKSYNC_MAPPING {
            m.insert(mapped, NetworkType::ZkSync);
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
        const AVALANCHE_MAPPING: [&str; 4] = ["CAVAX", "AVAXC", "AVAX", "AVALANCHE"];
        for mapped in AVALANCHE_MAPPING {
            m.insert(mapped, NetworkType::Avalanche);
        }
        // Metal DAO L2
        const METAL_MAPPING: [&str; 4] = ["METALDAOL2", "METAL", "METALDAO", "METALL2"];
        for mapped in METAL_MAPPING {
            m.insert(mapped, NetworkType::Metal);
        }

        // Cardano
        const CARDANO_MAPPING: [&str; 4] =
            ["ADA", "CARDANO", "CARDANONETWORK", "CARDANOBLOCKCHAIN"];
        for mapped in CARDANO_MAPPING {
            m.insert(mapped, NetworkType::Cardano);
        }

        // Dymension
        const DYMENSION_MAPPING: [&str; 4] = ["DYMEVM", "DYM", "DYMENSION", "DYMENSIONHUB"];
        for mapped in DYMENSION_MAPPING {
            m.insert(mapped, NetworkType::Dymension);
        }

        // Axelar
        const AXELAR_MAPPING: [&str; 3] = ["AXL", "AXELAR", "AXELARNETWORK"];
        for mapped in AXELAR_MAPPING {
            m.insert(mapped, NetworkType::Axelar);
        }

        // Moonriver (MOVR - токен Moonriver)
        const MOONRIVER_MAPPING: [&str; 3] = ["MOVR", "MOONRIVER", "MOONRIVERNETWORK"];
        for mapped in MOONRIVER_MAPPING {
            m.insert(mapped, NetworkType::Moonriver);
        }

        // BOB (Build on Bitcoin)
        const BOB_MAPPING: [&str; 4] = ["BOB", "BOBNETWORK", "BUILDONBITCOIN", "BOBCHAIN"];
        for mapped in BOB_MAPPING {
            m.insert(mapped, NetworkType::Bob);
        }
        // Arbitrum
        const ARBITRUM_MAPPING: [&str; 5] = ["ARBITRUM", "ARBITRUM ONE", "ARB", "ARB EVM", "ARBI"];
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
        // Lumia
        const LUMIA_MAPPING: [&str; 2] = ["LUMIA", "LUMIA NETWORK"];
        for mapped in LUMIA_MAPPING {
            m.insert(mapped, NetworkType::Lumia);
        }

        // IOTA
        const IOTA_MAPPING: [&str; 3] = ["IOTA", "IOTANETWORK", "IOTATANGLE"];
        for mapped in IOTA_MAPPING {
            m.insert(mapped, NetworkType::Iota);
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
        // Ronin
        const RONIN_MAPPING: [&str; 3] = ["RON", "RONIN", "RONIN NETWORK"];
        for mapped in RONIN_MAPPING {
            m.insert(mapped, NetworkType::Ronin);
        }
        // BounceBit
        const BOUNCEBIT_MAPPING: [&str; 3] = ["BB", "BOUNCEBIT", "BOUNCEBIT MAINNET"];
        for mapped in BOUNCEBIT_MAPPING {
            m.insert(mapped, NetworkType::BounceBit);
        }

        // Immutable
        const IMMUTABLE_MAPPING: [&str; 4] = ["IMMUTABLE", "IMMUTABLEX", "IMX", "IMMUTABLEZK"];
        for mapped in IMMUTABLE_MAPPING {
            m.insert(mapped, NetworkType::Immutable);
        }
        // Fantom
        const FANTOM_MAPPING: [&str; 3] = ["FTM", "FANTOM", "FANTOMOPERA"];
        for mapped in FANTOM_MAPPING {
            m.insert(mapped, NetworkType::Fantom);
        }

        // Enjin
        const ENJIN_MAPPING: [&str; 3] = ["ENJ", "ENJIN", "ENJIN NETWORK"];
        for mapped in ENJIN_MAPPING {
            m.insert(mapped, NetworkType::Enjin);
        }
        // Band Protocol
        const BAND_MAPPING: [&str; 3] = ["BAND", "BAND PROTOCOL", "BANDCHAIN"];
        for mapped in BAND_MAPPING {
            m.insert(mapped, NetworkType::Band);
        }

        m
    })
}
