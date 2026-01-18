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
        // STABLE (новый - возможно Stablecoin network?)
        const STABLE_MAPPING: [&str; 4] = [
            "STABLE",
            "STABLE NETWORK",
            "STABLE CHAIN",
            "STABLE PROTOCOL",
        ];
        for mapped in STABLE_MAPPING {
            m.insert(mapped, NetworkType::Stable);
        }

        // WEMIX (новый)
        const WEMIX_MAPPING: [&str; 4] =
            ["WEMIX", "WEMIX NETWORK", "WEMIX CHAIN", "WEMIX BLOCKCHAIN"];
        for mapped in WEMIX_MAPPING {
            m.insert(mapped, NetworkType::Wemix);
        }

        // Filecoin (новый)
        const FILECOIN_MAPPING: [&str; 5] =
            ["FIL", "FILEVM", "FILE COIN", "FILECOIN", "FIL NETWORK"];
        for mapped in FILECOIN_MAPPING {
            m.insert(mapped, NetworkType::Filecoin);
        }

        // Waves (новый)
        const WAVES_MAPPING: [&str; 4] = [
            "WAVES",
            "WAVES NETWORK",
            "WAVES PROTOCOL",
            "WAVES BLOCKCHAIN",
        ];
        for mapped in WAVES_MAPPING {
            m.insert(mapped, NetworkType::Waves);
        }

        // Neutron (NTRN) (новый)
        const NEUTRON_MAPPING: [&str; 4] =
            ["NTRN", "NEUTRON", "NEUTRON NETWORK", "NEUTRON PROTOCOL"];
        for mapped in NEUTRON_MAPPING {
            m.insert(mapped, NetworkType::Neutron);
        }

        // Merlin (новый)
        const MERLIN_MAPPING: [&str; 5] = [
            "MERLIN",
            "MERLIN NETWORK",
            "MERLIN CHAIN",
            "MERLIN LAYER",
            "MERLIN PROTOCOL",
        ];
        for mapped in MERLIN_MAPPING {
            m.insert(mapped, NetworkType::Merlin);
        }

        // Aurora (новый)
        const AURORA_MAPPING: [&str; 5] = [
            "AURORAEVM",
            "AURORA",
            "AURORA CHAIN",
            "AURORA EVM",
            "AURORA NETWORK",
        ];
        for mapped in AURORA_MAPPING {
            m.insert(mapped, NetworkType::Aurora);
        }

        // NEM (XEM) (новый)
        const NEM_MAPPING: [&str; 5] = [
            "XEM",
            "NEM",
            "NEM NETWORK",
            "NEM BLOCKCHAIN",
            "NEW ECONOMY MOVEMENT",
        ];
        for mapped in NEM_MAPPING {
            m.insert(mapped, NetworkType::Nem);
        }

        // SKALE (новый)
        const SKALE_MAPPING: [&str; 4] =
            ["SKALE", "SKALE NETWORK", "SKALE CHAIN", "SKALE PROTOCOL"];
        for mapped in SKALE_MAPPING {
            m.insert(mapped, NetworkType::Skale);
        }

        // Solar (SXP) (новый)
        const SOLAR_MAPPING: [&str; 5] = [
            "SXP",
            "SOLAR",
            "SOLAR NETWORK",
            "SXP NETWORK",
            "SOLAR BLOCKCHAIN",
        ];
        for mapped in SOLAR_MAPPING {
            m.insert(mapped, NetworkType::Solar);
        }

        // Nillion (новый)
        const NILLION_MAPPING: [&str; 4] = [
            "NILLION",
            "NILLION NETWORK",
            "NILLION PROTOCOL",
            "NILLION CHAIN",
        ];
        for mapped in NILLION_MAPPING {
            m.insert(mapped, NetworkType::Nillion);
        }

        // Elastos (новый)
        const ELASTOS_MAPPING: [&str; 5] = [
            "ELASTOS",
            "ELA",
            "ELASTOS NETWORK",
            "ELASTOS CHAIN",
            "ELASTOS BLOCKCHAIN",
        ];
        for mapped in ELASTOS_MAPPING {
            m.insert(mapped, NetworkType::Elastos);
        }

        // HyperEVM (новый)
        const HYPEREVM_MAPPING: [&str; 4] =
            ["HYPEREVM", "HYPER EVM", "HYPER NETWORK", "HYPER CHAIN"];
        for mapped in HYPEREVM_MAPPING {
            m.insert(mapped, NetworkType::HyperEVM);
        }

        // Manta Network (новый)
        const MANTA_MAPPING: [&str; 6] = [
            "MANTA",
            "MANTA NETWORK",
            "MANTA CHAIN",
            "MANTA L2",
            "MANTA PACIFIC",
            "MANTA MAINNET", // Добавил новое название
        ];
        for mapped in MANTA_MAPPING {
            m.insert(mapped, NetworkType::Manta);
        }
        const MANTLE_MAPPING: [&str; 7] = [
            "MANTLE",
            "MANTLE MAINNET",
            "MANTLE NETWORK",
            "MANTLE CHAIN",
            "MANTLE L2",
            "MANTLE PROTOCOL",
            "MANTLE TOKEN", // Добавил новое название
        ];
        for mapped in MANTLE_MAPPING {
            m.insert(mapped, NetworkType::Mantle);
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
        // FOGO (новый)
        const FOGO_MAPPING: [&str; 3] = ["FOGO", "FOGO NETWORK", "FOGO CHAIN"];
        for mapped in FOGO_MAPPING {
            m.insert(mapped, NetworkType::Fogo);
        }

        // Frax (новый)
        const FRAX_MAPPING: [&str; 5] = [
            "FRAX",
            "FRAX NETWORK",
            "FRAX PROTOCOL",
            "FRAX FINANCE",
            "FRAXTOKEN",
        ];
        for mapped in FRAX_MAPPING {
            m.insert(mapped, NetworkType::Frax);
        }

        // Siacoin (новый)
        const SIACOIN_MAPPING: [&str; 4] = ["SC", "SIACOIN", "SIA", "SIA NETWORK"];
        for mapped in SIACOIN_MAPPING {
            m.insert(mapped, NetworkType::Siacoin);
        }

        // VeChain (уже есть? если нет - новый)
        const VECHAIN_MAPPING: [&str; 4] = ["VECHAIN", "VET", "VE CHAIN", "VE CHAIN THOR"];
        for mapped in VECHAIN_MAPPING {
            m.insert(mapped, NetworkType::VeChain);
        }

        // Theta (новый)
        const THETA_MAPPING: [&str; 5] = [
            "THETA",
            "THETA TOKEN",
            "THETA NETWORK",
            "THETA BLOCKCHAIN",
            "THETA PROTOCOL",
        ];
        for mapped in THETA_MAPPING {
            m.insert(mapped, NetworkType::Theta);
        }

        // Fetch.ai (новый)
        const FETCHAI_MAPPING: [&str; 5] =
            ["FET", "FETCH.AI", "FETCH AI", "FETCH", "FETCH NETWORK"];
        for mapped in FETCHAI_MAPPING {
            m.insert(mapped, NetworkType::FetchAi);
        }

        // Arweave (новый)
        const ARWEAVE_MAPPING: [&str; 4] = ["AR", "ARWEAVE", "AR WEAVE", "AR PERMAWEB"];
        for mapped in ARWEAVE_MAPPING {
            m.insert(mapped, NetworkType::Arweave);
        }

        // ROSE (Oasis Network) (новый)
        const OASIS_MAPPING: [&str; 5] = [
            "ROSE",
            "OASIS",
            "OASIS NETWORK",
            "OASIS PROTOCOL",
            "OASIS ROSE",
        ];
        for mapped in OASIS_MAPPING {
            m.insert(mapped, NetworkType::Oasis);
        }

        // Cosmos (ATOM) (новый)
        const COSMOS_MAPPING: [&str; 5] = [
            "ATOM",
            "COSMOS",
            "COSMOS NETWORK",
            "COSMOS HUB",
            "COSMOS PROTOCOL",
        ];
        for mapped in COSMOS_MAPPING {
            m.insert(mapped, NetworkType::Cosmos);
        }

        // Fraxtal (Frax L2) (новый)
        const FRAXTAL_MAPPING: [&str; 4] = ["FRAXTAL", "FRAX L2", "FRAX CHAIN", "FRAXTAL L2"];
        for mapped in FRAXTAL_MAPPING {
            m.insert(mapped, NetworkType::Fraxtal);
        }

        // Ravencoin (новый)
        const RAVENCOIN_MAPPING: [&str; 4] = ["RVN", "RAVENCOIN", "RAVEN COIN", "RAVEN"];
        for mapped in RAVENCOIN_MAPPING {
            m.insert(mapped, NetworkType::Ravencoin);
        }

        // Asset Hub Polkadot (Statemint) (новый)
        const ASSET_HUB_POLKADOT_MAPPING: [&str; 6] = [
            "STATEMINT",
            "ASSET HUB",
            "ASSET HUB POLKADOT",
            "POLKADOT ASSET HUB",
            "STATEMINT ASSET HUB",
            "ASSET HUB DOT",
        ];
        for mapped in ASSET_HUB_POLKADOT_MAPPING {
            m.insert(mapped, NetworkType::AssetHubPolkadot);
        }

        // Hedera (HBAR) (новый)
        const HEDERA_MAPPING: [&str; 5] = [
            "HBAR",
            "HEDERA",
            "HEDERA HASHGRAPH",
            "HEDERA NETWORK",
            "HEDERA TOKEN",
        ];
        for mapped in HEDERA_MAPPING {
            m.insert(mapped, NetworkType::Hedera);
        }

        // Algorand (новый)
        const ALGORAND_MAPPING: [&str; 4] =
            ["ALGO", "ALGORAND", "ALGO NETWORK", "ALGORAND BLOCKCHAIN"];
        for mapped in ALGORAND_MAPPING {
            m.insert(mapped, NetworkType::Algorand);
        }

        // Conflux (CFX) (новый)
        const CONFLUX_MAPPING: [&str; 6] = [
            "CFX",
            "CFXCORE",
            "CFX ESPACE",
            "CFX CORE",
            "CONFLUX",
            "CONFLUX NETWORK",
        ];
        for mapped in CONFLUX_MAPPING {
            m.insert(mapped, NetworkType::Conflux);
        }
        // Nano (новый)
        const NANO_MAPPING: [&str; 4] =
            ["NANO", "NANO COIN", "NANO NETWORK", "NANO CRYPTOCURRENCY"];
        for mapped in NANO_MAPPING {
            m.insert(mapped, NetworkType::Nano);
        }

        // eCash (XEC) (новый)
        const ECASH_MAPPING: [&str; 5] = ["XEC", "ECASH", "E CASH", "BITCOIN CASH ABC", "BCHA"];
        for mapped in ECASH_MAPPING {
            m.insert(mapped, NetworkType::ECash);
        }

        // Init (новый - возможно Initial?)
        const INIT_MAPPING: [&str; 4] = ["INIT", "INITIAL", "INIT NETWORK", "INITIAL NETWORK"];
        for mapped in INIT_MAPPING {
            m.insert(mapped, NetworkType::Init);
        }

        // Stellar (XLM) (новый)
        const STELLAR_MAPPING: [&str; 4] = ["XLM", "STELLAR", "STELLAR LUMENS", "STELLAR NETWORK"];
        for mapped in STELLAR_MAPPING {
            m.insert(mapped, NetworkType::Stellar);
        }

        // DigiByte (новый)
        const DIGIBYTE_MAPPING: [&str; 4] = ["DGB", "DIGIBYTE", "DIGI BYTE", "DIGIBYTE NETWORK"];
        for mapped in DIGIBYTE_MAPPING {
            m.insert(mapped, NetworkType::DigiByte);
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
        const ASSET_HUB_KUSAMA_MAPPING: [&str; 4] = [
            "ASSET HUB KUSAMA",
            "KUSAMA ASSET HUB",
            "KUSAMA HUB",
            "AssetHubKusama",
        ];
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
        const ZEROG_MAPPING: [&str; 2] = ["0G", "ZEROGRAVITY"];
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
        const BERA_MAPPING: [&str; 2] = ["BERA", "BERACHAIN"];
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
        const BITCOIN_MAPPING: [&str; 9] = [
            "BTC",
            "BITCOIN",
            "BTCBRC",
            "BRC20",
            "ORDIBTC",
            "ORDI BRC20",
            "ORDI-BRC20",
            "BRC20",
            "BITCOIN BRC20",
        ];
        for mapped in BITCOIN_MAPPING {
            m.insert(mapped, NetworkType::Bitcoin);
        }
        // Klaytn (новый)
        const KLAYTN_MAPPING: [&str; 5] = [
            "KLAY",
            "KLAYTN",
            "KLAY NETWORK",
            "KLAYTN NETWORK",
            "KLAYTN CHAIN",
        ];
        for mapped in KLAYTN_MAPPING {
            m.insert(mapped, NetworkType::Klaytn);
        }

        // Plasma (новый)
        const PLASMA_MAPPING: [&str; 5] = [
            "PLASMA",
            "PLASMA NETWORK",
            "PLASMA CHAIN",
            "PLASMA PROTOCOL",
            "PLASMA FINANCE",
        ];
        for mapped in PLASMA_MAPPING {
            m.insert(mapped, NetworkType::Plasma);
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
        // Starknet (уже существует)
        const STARKNET_MAPPING: [&str; 6] = [
            "STARKNET",
            "STARK NET",
            "STARKWARE",
            "STARK NETWORK",
            "STARK CHAIN",
            "STARK", // Добавил новое название
        ];
        for mapped in STARKNET_MAPPING {
            m.insert(mapped, NetworkType::Starknet);
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
        // Morph (новый)
        const MORPH_MAPPING: [&str; 5] = [
            "MORPH",
            "MORPH NETWORK",
            "MORPH CHAIN",
            "MORPH L2",
            "MORPH PROTOCOL",
        ];
        for mapped in MORPH_MAPPING {
            m.insert(mapped, NetworkType::Morph);
        }
        // Harmony
        const HARMONY_MAPPING: [&str; 2] = ["ONE", "HARMONY"];
        for mapped in HARMONY_MAPPING {
            m.insert(mapped, NetworkType::Harmony);
        }
        // zkSync Era
        const ZKSYNC_ERA_MAPPING: [&str; 7] = [
            "ZKV2",
            "ZKSYNC ERA",
            "ZKSYNC",
            "ZK SYNC",
            "ZKSYNC V2",
            "ZK SYNC ERA",
            "ZKSYNCERA", // Добавил вариант без пробела
        ];
        for mapped in ZKSYNC_ERA_MAPPING {
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
        const AVALANCHE_MAPPING: [&str; 9] = [
            "CAVAX",
            "AVAXC",
            "AVAX",
            "AVALANCHE",
            "AVAX C-CHAIN",
            "AVAX_C",
            "AVAX C CHAIN",
            "AVALANCHE C-CHAIN",
            "AVALANCHE C CHAIN",
        ];
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
        const ARBITRUM_MAPPING: [&str; 7] = [
            "ARBITRUMONE",
            "ARBITRUM ONE",
            "ARBITRUM",
            "ARBITRUM L2",
            "ARBITRUM MAINNET",
            "ARBITRUM1", // Добавил новое название
            "ARB",
        ];
        for mapped in ARBITRUM_MAPPING {
            m.insert(mapped, NetworkType::Arbitrum);
        }

        // Arbitrum Nova (новый)
        const ARBITRUM_NOVA_MAPPING: [&str; 6] = [
            "ARBINOVA",
            "ARBITRUM NOVA",
            "ARBITRUM NOVA L2",
            "NOVA",
            "ARBITRUM NOVA NETWORK",
            "ARBITRUMNOVA", // Добавил вариант без пробела
        ];
        for mapped in ARBITRUM_NOVA_MAPPING {
            m.insert(mapped, NetworkType::ArbitrumNova);
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
        // NEAR Protocol (уже есть)
        const NEAR_MAPPING: [&str; 7] = [
            "NEAR",
            "NEAR PROTOCOL",
            "NEAR CHAIN",
            "NEAR NETWORK",
            "NEAR BLOCKCHAIN", // Добавил новое название
            "NEAR TOKEN",      // Добавил новое название
            "NEARProtocol",
        ];
        for mapped in NEAR_MAPPING {
            m.insert(mapped, NetworkType::Near);
        }
        // SEI
        const SEI_MAPPING: [&str; 5] =
            ["SEI", "SEIEVM", "SEI NETWORK", "SEI CHAIN", "SEI PROTOCOL"];
        for mapped in SEI_MAPPING {
            m.insert(mapped, NetworkType::Sei);
        }
        // Litecoin (дополнение)
        const LITECOIN_MAPPING: [&str; 4] = ["LTC", "LITECOIN", "LITE COIN", "LITE"];
        for mapped in LITECOIN_MAPPING {
            m.insert(mapped, NetworkType::Litecoin);
        }

        const LIGHTNING_MAPPING: [&str; 5] = [
            "BTCLN",
            "LIGHTNING",
            "LIGHTNING NETWORK",
            "BITCOIN LIGHTNING",
            "LN NETWORK",
        ];
        for mapped in LIGHTNING_MAPPING {
            m.insert(mapped, NetworkType::Lightning);
        }
        // Bitcoin SegWit (дополнение к Bitcoin)
        const BITCOIN_SEGWIT_MAPPING: [&str; 5] = [
            "SEGWITBTC",
            "BTC SEGWIT",
            "BITCOIN SEGWIT",
            "SEGWIT",
            "BTC (SEGWIT)",
        ];
        for mapped in BITCOIN_SEGWIT_MAPPING {
            m.insert(mapped, NetworkType::Bitcoin);
        }
        const KCC_MAPPING: [&str; 5] = [
            "KCC",
            "KCS",
            "KUCOIN COMMUNITY CHAIN",
            "KUCOIN CHAIN",
            "KCC NETWORK",
        ];
        for mapped in KCC_MAPPING {
            m.insert(mapped, NetworkType::Kcc);
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
        // Syscoin (новый)
        const SYSCOIN_MAPPING: [&str; 4] = ["SYS", "SYSCOIN", "SYS COIN", "SYS NETWORK"];
        for mapped in SYSCOIN_MAPPING {
            m.insert(mapped, NetworkType::Syscoin);
        }

        // Kava (новый)
        const KAVA_MAPPING: [&str; 5] =
            ["KAVA", "KAVAEVM", "KAVA EVM", "KAVA NETWORK", "KAVA CHAIN"];
        for mapped in KAVA_MAPPING {
            m.insert(mapped, NetworkType::Kava);
        }

        // IoTeX (новый)
        const IOTEX_MAPPING: [&str; 4] = ["IOTX", "IOTEX", "IOT EX", "INTERNET OF THINGS EX"];
        for mapped in IOTEX_MAPPING {
            m.insert(mapped, NetworkType::IoTeX);
        }

        // Internet Computer (новый)
        const INTERNET_COMPUTER_MAPPING: [&str; 6] = [
            "ICP",
            "INTERNET COMPUTER",
            "ICP NETWORK",
            "INTERNET COMPUTER PROTOCOL",
            "DFINITY",
            "DFINITY ICP",
        ];
        for mapped in INTERNET_COMPUTER_MAPPING {
            m.insert(mapped, NetworkType::InternetComputer);
        }

        // Acala (новый)
        const ACALA_MAPPING: [&str; 4] = ["ACA", "ACALA", "ACALA NETWORK", "ACALA PROTOCOL"];
        for mapped in ACALA_MAPPING {
            m.insert(mapped, NetworkType::Acala);
        }

        // FitFi (Step App) (новый)
        const FITFI_MAPPING: [&str; 4] = ["FITFI", "STEP APP", "FITFI NETWORK", "STEP NETWORK"];
        for mapped in FITFI_MAPPING {
            m.insert(mapped, NetworkType::FitFi);
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
        const DOGECOIN_MAPPING: [&str; 5] = [
            "DOGE",
            "DOGECOIN",
            "DOGE COIN",
            "DOGE CHAIN",
            "DOGECOIN NETWORK", // Добавил новое название
        ];
        for mapped in DOGECOIN_MAPPING {
            m.insert(mapped, NetworkType::Dogecoin);
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
