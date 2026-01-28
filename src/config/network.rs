use std::{collections::HashMap, sync::OnceLock};

use crate::core::types::network::NetworkType;

static NETWORK_MAP: OnceLock<HashMap<&'static str, NetworkType>> = OnceLock::new();

pub fn get_map() -> &'static HashMap<&'static str, NetworkType> {
    NETWORK_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        // Syscoin Rollux (L2 на Syscoin)
        const SYSCOIN_ROLLUX_MAPPING: [&str; 5] = [
            "SYSROLLUX",
            "SYS ROLLUX",
            "ROLLUX",
            "SYSCOIN ROLLUX",
            "SYSCOIN L2",
        ];
        for mapped in SYSCOIN_ROLLUX_MAPPING {
            m.insert(mapped, NetworkType::SyscoinRollux);
        }

        // Injective (дополнение к существующему)
        const INJECTIVE_MAPPING: [&str; 5] = [
            "INJ",
            "INJECTIVE",
            "INJECTIVE PROTOCOL",
            "INJECTIVE CHAIN",
            "INJECTIVE NETWORK",
        ];
        for mapped in INJECTIVE_MAPPING {
            m.insert(mapped, NetworkType::Injective);
        }

        // THORChain (RUNE)
        const THORCHAIN_MAPPING: [&str; 5] = [
            "RUNE",
            "THORCHAIN",
            "THOR CHAIN",
            "THOR CHAIN RUNE",
            "THOR PROTOCOL",
        ];
        for mapped in THORCHAIN_MAPPING {
            m.insert(mapped, NetworkType::Thorchain);
        }

        // Bitcoin Runes (новый стандарт на Bitcoin)
        const BITCOIN_RUNES_MAPPING: [&str; 5] = [
            "BTCRUNES",
            "BITCOIN RUNES",
            "RUNES",
            "BTC RUNES",
            "BITCOIN RUNES PROTOCOL",
        ];
        for mapped in BITCOIN_RUNES_MAPPING {
            m.insert(mapped, NetworkType::Bitcoin); // Или NetworkType::BitcoinRunes если нужно отдельно
        }

        // Sapphire (Oasis конфиденциальный парачейн)
        const SAPPHIRE_MAPPING: [&str; 5] = [
            "SAPPHIRE",
            "SAPPHIRE NETWORK",
            "OASIS SAPPHIRE",
            "SAPPHIRE PARACHAIN",
            "SAPPHIRE CHAIN",
        ];
        for mapped in SAPPHIRE_MAPPING {
            m.insert(mapped, NetworkType::Sapphire);
        }

        // Zcash
        const ZCASH_MAPPING: [&str; 4] = ["ZEC", "ZCASH", "Z CASH", "ZCASH NETWORK"];
        for mapped in ZCASH_MAPPING {
            m.insert(mapped, NetworkType::Zcash);
        }

        // Klever (KLV)
        const KLEVER_MAPPING: [&str; 5] = [
            "KLV",
            "KLEVER",
            "KLEVER CHAIN",
            "KLEVER NETWORK",
            "KLEVER BLOCKCHAIN",
        ];
        for mapped in KLEVER_MAPPING {
            m.insert(mapped, NetworkType::Klever);
        }

        // WAX (Worldwide Asset eXchange)
        const WAX_MAPPING: [&str; 7] = [
            "WAX",
            "WAXP",
            "WAX NETWORK",
            "WORLDWIDE ASSET EXCHANGE",
            "WAX BLOCKCHAIN",
            "WAX CHAIN",
            "WAX TOKEN",
        ];
        for mapped in WAX_MAPPING {
            m.insert(mapped, NetworkType::Wax);
        }

        // Kaia (бывший Klaytn, переименован)
        const KAIA_MAPPING: [&str; 6] = [
            "KAIA",
            "KLAYTN",
            "KLAY",
            "KAIA NETWORK",
            "KLAYTN NETWORK",
            "KAIA CHAIN",
        ];
        for mapped in KAIA_MAPPING {
            m.insert(mapped, NetworkType::Kaia);
        }

        // Manta Atlantic (Zk-rollup Polkadot)
        const MANTA_ATLANTIC_MAPPING: [&str; 5] = [
            "MANTADOT",
            "MANTA ATLANTIC",
            "MANTA DOT",
            "ATLANTIC",
            "MANTA POLKADOT",
        ];
        for mapped in MANTA_ATLANTIC_MAPPING {
            m.insert(mapped, NetworkType::MantaAtlantic);
        }

        // ZIGChain (Zignaly)
        const ZIGCHAIN_MAPPING: [&str; 5] =
            ["ZIG", "ZIGCHAIN", "ZIG CHAIN", "ZIGNALY", "ZIGNALY CHAIN"];
        for mapped in ZIGCHAIN_MAPPING {
            m.insert(mapped, NetworkType::ZigChain);
        }

        // Aptos
        const APTOS_MAPPING: [&str; 4] =
            ["APTOS", "APTOS NETWORK", "APTOS CHAIN", "APTOS BLOCKCHAIN"];
        for mapped in APTOS_MAPPING {
            m.insert(mapped, NetworkType::Aptos);
        }

        // Chromia
        const CHROMIA_MAPPING: [&str; 5] = [
            "CHR",
            "CHROMIA",
            "CHROMIA NETWORK",
            "CHROMIA PLATFORM",
            "CHROMIA BLOCKCHAIN",
        ];
        for mapped in CHROMIA_MAPPING {
            m.insert(mapped, NetworkType::Chromia);
        }

        // Tanssi (Appchain Infrastructure)
        const TANSSI_MAPPING: [&str; 5] = [
            "TANSSI",
            "TANSSI NETWORK",
            "TANSSI PROTOCOL",
            "TANSSI CHAIN",
            "TANSSI APPCHAIN",
        ];
        for mapped in TANSSI_MAPPING {
            m.insert(mapped, NetworkType::Tanssi);
        }
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

        // Manta Network (добавляем новый вариант)
        const MANTA_MAPPING: [&str; 7] = [
            "MANTA",
            "MANTA NETWORK",
            "MANTA CHAIN",
            "MANTA L2",
            "MANTA PACIFIC",
            "MANTA MAINNET",
            "MANTANETWORK", // Добавляем новый вариант без пробела
        ];
        for mapped in MANTA_MAPPING {
            m.insert(mapped, NetworkType::Manta);
        }

        // MilkyWay (MILK) (новый)
        const MILKYWAY_MAPPING: [&str; 4] = ["MILK", "MILKYWAY", "MILKY WAY", "MILKY WAY NETWORK"];
        for mapped in MILKYWAY_MAPPING {
            m.insert(mapped, NetworkType::MilkyWay);
        }

        // Flow (новый)
        const FLOW_MAPPING: [&str; 4] =
            ["FLOW", "FLOW NETWORK", "FLOW BLOCKCHAIN", "FLOW PROTOCOL"];
        for mapped in FLOW_MAPPING {
            m.insert(mapped, NetworkType::Flow);
        }

        // Qtum (новый)
        const QTUM_MAPPING: [&str; 4] = ["QTUM", "QTUM NETWORK", "QTUM BLOCKCHAIN", "QTUM CHAIN"];
        for mapped in QTUM_MAPPING {
            m.insert(mapped, NetworkType::Qtum);
        }

        // V SYSTEMS (VSYS) (новый)
        const VSYSTEMS_MAPPING: [&str; 5] = [
            "VSYS",
            "V SYSTEMS",
            "VSYSTEMS",
            "V SYSTEMS BLOCKCHAIN",
            "VSYS NETWORK",
        ];
        for mapped in VSYSTEMS_MAPPING {
            m.insert(mapped, NetworkType::VSystems);
        }

        // Beldex (BDX) (новый)
        const BELDEX_MAPPING: [&str; 5] = [
            "BDX",
            "BELDEX",
            "BELDEX NETWORK",
            "BELDEX CHAIN",
            "BELDEX BLOCKCHAIN",
        ];
        for mapped in BELDEX_MAPPING {
            m.insert(mapped, NetworkType::Beldex);
        }

        // Secret Network (SCRT) (новый)
        const SECRET_MAPPING: [&str; 6] = [
            "SCRT",
            "SECRET",
            "SECRET NETWORK",
            "SECRET PROTOCOL",
            "SECRET CHAIN",
            "SCRT NETWORK",
        ];
        for mapped in SECRET_MAPPING {
            m.insert(mapped, NetworkType::Secret);
        }

        // Astar Network (ASTR) (новый)
        const ASTAR_MAPPING: [&str; 11] = [
            "ASTR",
            "ASTAR",
            "ASTAR NETWORK",
            "ASTAR CHAIN",
            "ASTAR BLOCKCHAIN",
            "ASTAREVM",
            "ASTAR EVM",
            "ASTAR EVM CHAIN",
            "ASTAR EVM NETWORK",
            "ASTAR EVM LAYER",
            "ASTAR EVM PROTOCOL",
        ];
        for mapped in ASTAR_MAPPING {
            m.insert(mapped, NetworkType::Astar);
        }

        // Supra (новый)
        const SUPRA_MAPPING: [&str; 5] = [
            "SUPRA",
            "SUPRA NETWORK",
            "SUPRA CHAIN",
            "SUPRA PROTOCOL",
            "SUPRA ORACLE",
        ];
        for mapped in SUPRA_MAPPING {
            m.insert(mapped, NetworkType::Supra);
        }

        // Canton Network (новый)
        const CANTON_MAPPING: [&str; 5] = [
            "CANTON",
            "CANTON NETWORK",
            "CANTON PROTOCOL",
            "CANTON CHAIN",
            "CANTON BLOCKCHAIN",
        ];
        for mapped in CANTON_MAPPING {
            m.insert(mapped, NetworkType::Canton);
        }

        // OORT (новый)
        const OORT_MAPPING: [&str; 5] = [
            "OORT",
            "OORT NETWORK",
            "OORT CHAIN",
            "OORT PROTOCOL",
            "OORT CLOUD",
        ];
        for mapped in OORT_MAPPING {
            m.insert(mapped, NetworkType::Oort);
        }

        // Matrix AI Network (MAN) (новый)
        const MATRIX_MAPPING: [&str; 6] = [
            "MAN",
            "MATRIX",
            "MATRIX AI NETWORK",
            "MATRIX NETWORK",
            "MATRIX CHAIN",
            "MATRIX AI",
        ];
        for mapped in MATRIX_MAPPING {
            m.insert(mapped, NetworkType::Matrix);
        }

        // XION (новый)
        const XION_MAPPING: [&str; 5] = [
            "XION",
            "XION NETWORK",
            "XION CHAIN",
            "XION PROTOCOL",
            "XION BLOCKCHAIN",
        ];
        for mapped in XION_MAPPING {
            m.insert(mapped, NetworkType::Xion);
        }

        // XFI (Xfinite Entertainment Token) (новый)
        const XFI_MAPPING: [&str; 5] = [
            "XFI",
            "XFITE",
            "XFINITE",
            "XFINITE ENTERTAINMENT",
            "XFI NETWORK",
        ];
        for mapped in XFI_MAPPING {
            m.insert(mapped, NetworkType::Xfi);
        }

        // U2U (новый)
        const U2U_MAPPING: [&str; 5] = [
            "U2U",
            "U2U NETWORK",
            "U2U CHAIN",
            "U2U BLOCKCHAIN",
            "U2U PROTOCOL",
        ];
        for mapped in U2U_MAPPING {
            m.insert(mapped, NetworkType::U2U);
        }

        // Mintlayer (ML) (новый)
        const MINTLAYER_MAPPING: [&str; 5] = [
            "ML",
            "MINTLAYER",
            "MINT LAYER",
            "MINTLAYER NETWORK",
            "MINTLAYER CHAIN",
        ];
        for mapped in MINTLAYER_MAPPING {
            m.insert(mapped, NetworkType::Mintlayer);
        }

        // Xai (новый)
        const XAI_MAPPING: [&str; 5] = [
            "XAI",
            "XAI CHAIN",
            "XAI NETWORK",
            "XAI BLOCKCHAIN",
            "XAI PROTOCOL",
        ];
        for mapped in XAI_MAPPING {
            m.insert(mapped, NetworkType::Xai);
        }

        // Smart Chain (SMART) (новый)
        const SMART_CHAIN_MAPPING: [&str; 5] = [
            "SMART",
            "SMART CHAIN",
            "SMART NETWORK",
            "SMART BLOCKCHAIN",
            "SMART PROTOCOL",
        ];
        for mapped in SMART_CHAIN_MAPPING {
            m.insert(mapped, NetworkType::SmartChain);
        }

        // Meter (MTR) (новый)
        const METER_MAPPING: [&str; 5] = [
            "MTR",
            "METER",
            "METER NETWORK",
            "METER CHAIN",
            "METER BLOCKCHAIN",
        ];
        for mapped in METER_MAPPING {
            m.insert(mapped, NetworkType::Meter);
        }

        // Meter Governance (MTRG) (новый)
        const METERG_MAPPING: [&str; 5] = [
            "MTRG",
            "METER GOVERNANCE",
            "METER GOVERNANCE TOKEN",
            "METER GOV",
            "MTRG NETWORK",
        ];
        for mapped in METERG_MAPPING {
            m.insert(mapped, NetworkType::MeterGovernance);
        }

        // MAP Protocol (MAPO) (новый)
        const MAP_MAPPING: [&str; 6] = [
            "MAPO",
            "MAP",
            "MAP PROTOCOL",
            "MAP NETWORK",
            "MAP CHAIN",
            "MAP BLOCKCHAIN",
        ];
        for mapped in MAP_MAPPING {
            m.insert(mapped, NetworkType::MapProtocol);
        }

        // HyperEVM (новый)
        const HYPEREVM_MAPPING: [&str; 4] =
            ["HYPEREVM", "HYPER EVM", "HYPER NETWORK", "HYPER CHAIN"];
        for mapped in HYPEREVM_MAPPING {
            m.insert(mapped, NetworkType::HyperEVM);
        }

        const MANTLE_MAPPING: [&str; 8] = [
            "MNT",
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
        const ASSET_HUB_POLKADOT_MAPPING: [&str; 10] = [
            "STATEMINT",
            "ASSET HUB",
            "ASSET HUB POLKADOT",
            "POLKADOT ASSET HUB",
            "POLKADOTASSETHUB",
            "STATEMINT ASSET HUB",
            "ASSET HUB DOT",
            "DOTAH",
            "POLKADOT ASSETHUB",
            "DOTASSETHUB",
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
        const STELLAR_MAPPING: [&str; 5] =
            ["XLM", "STELLAR", "STELLAR LUMENS", "STELLAR NETWORK", "S"];
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
            "ASSETHUBKUSAMA",
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
        const ZKSYNC_ERA_MAPPING: [&str; 9] = [
            "ZKSYNC2",
            "ZKS20",
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
        const AVALANCHE_MAPPING: [&str; 11] = [
            "CAVAX",
            "AVAXC",
            "AVAX",
            "AVALANCHE",
            "AVAX C-CHAIN",
            "AVAX_C",
            "AVAX_CCHAIN",
            "AVALANCHE C-CHAIN",
            "AVALANCHE C CHAIN",
            "AVAX_XCHAIN",
            "AVAXC-CHAIN",
        ];
        for mapped in AVALANCHE_MAPPING {
            m.insert(mapped, NetworkType::Avalanche);
        }
        // Metal DAO L2
        const METAL_MAPPING: [&str; 6] = [
            "METALDAOL2",
            "METAL",
            "METALDAO",
            "METALL2",
            "METAL L2",
            "METAL DAO L2",
        ];
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
        const AXELAR_MAPPING: [&str; 5] =
            ["WAXL", "AXL", "AXELAR", "AXELARNETWORK", "AXELAR MAINNET"];
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
        const METIS_MAPPING: [&str; 4] = ["METIS TOKEN", "METIS", "METIS NETWORK", "METISTOKEN"];
        for mapped in METIS_MAPPING {
            m.insert(mapped, NetworkType::Metis);
        }
        // NEAR Protocol (уже есть)
        const NEAR_MAPPING: [&str; 7] = [
            "NEAR",
            "NEARPROTOCOL",
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

        // ZIL
        const ZIL_MAPPING: [&str; 2] = ["ZIL", "ZILLIQA"];
        for mapped in ZIL_MAPPING {
            m.insert(mapped, NetworkType::Zilliqa);
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
        const NEO_MAPPING: [&str; 6] = ["GAS", "NEO GAS", "NEO3", "NEO", "NEON3", "NEO BLOCKCHAIN"];
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

        // ZetaChain (ZETA) (новый)
        const ZETACHAIN_MAPPING: [&str; 8] = [
            "ZETA",
            "ZETACHAIN",
            "ZETA CHAIN",
            "ZETACHAIN NETWORK",
            "ZETACHAIN PROTOCOL",
            "ZETAEVM",
            "ZETACHAIN EVM",
            "ZETACHAIN BLOCKCHAIN",
        ];
        for mapped in ZETACHAIN_MAPPING {
            m.insert(mapped, NetworkType::ZetaChain);
        }

        // Stratos (STOS) (новый)
        const STRATOS_MAPPING: [&str; 6] = [
            "STOS",
            "STRATOS",
            "STRATOS NETWORK",
            "STRATOS CHAIN",
            "STRATOS BLOCKCHAIN",
            "STRATOS PROTOCOL",
        ];
        for mapped in STRATOS_MAPPING {
            m.insert(mapped, NetworkType::Stratos);
        }

        // Bitcoin Cash (BCH) - ОТДЕЛЬНАЯ СЕТЬ от Bitcoin!
        const BITCOIN_CASH_MAPPING: [&str; 7] = [
            "BCH",
            "BITCOIN CASH",
            "BITCOINCASH",
            "BCH NETWORK",
            "BCH CHAIN",
            "BCH BLOCKCHAIN",
            "BITCOIN CASH NETWORK",
        ];
        for mapped in BITCOIN_CASH_MAPPING {
            m.insert(mapped, NetworkType::BitcoinCash);
        }

        // Taiko (TAIKO) (новый)
        const TAIKO_MAPPING: [&str; 7] = [
            "TAIKO",
            "TAIKOETH",
            "TAIKO ETH",
            "TAIKO NETWORK",
            "TAIKO CHAIN",
            "TAIKO BLOCKCHAIN",
            "TAIKO L2",
        ];
        for mapped in TAIKO_MAPPING {
            m.insert(mapped, NetworkType::Taiko);
        }

        // Arch (ARCH) (новый)
        const ARCH_MAPPING: [&str; 5] = [
            "ARCH",
            "ARCH NETWORK",
            "ARCH CHAIN",
            "ARCH BLOCKCHAIN",
            "ARCH PROTOCOL",
        ];
        for mapped in ARCH_MAPPING {
            m.insert(mapped, NetworkType::Arch);
        }

        // My Master War (MAT) (новый)
        const MAT_MAPPING: [&str; 6] = [
            "MAT",
            "MY MASTER WAR",
            "MAT NETWORK",
            "MAT CHAIN",
            "MAT BLOCKCHAIN",
            "MAT PROTOCOL",
        ];
        for mapped in MAT_MAPPING {
            m.insert(mapped, NetworkType::Mat);
        }

        // opBNB (новый - Optimistic Rollup на BSC)
        const OPBNB_MAPPING: [&str; 6] = [
            "OPBNB",
            "OP BNB",
            "OPBNB NETWORK",
            "OPBNB CHAIN",
            "OPBNB L2",
            "BINANCE OP ROLLUP",
        ];
        for mapped in OPBNB_MAPPING {
            m.insert(mapped, NetworkType::OpBNB);
        }

        // Mitosis (MITO) (новый)
        const MITOSIS_MAPPING: [&str; 6] = [
            "MITO",
            "MITOSIS",
            "MITOSIS NETWORK",
            "MITOSIS CHAIN",
            "MITOSIS BLOCKCHAIN",
            "MITOSIS PROTOCOL",
        ];
        for mapped in MITOSIS_MAPPING {
            m.insert(mapped, NetworkType::Mitosis);
        }

        const DOGECOIN_MAPPING: [&str; 6] = [
            "DOGE",
            "DOGECOIN",
            "DOGE COIN",
            "DOGE CHAIN",
            "DOGECHAIN",
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
        const ONTOLOGY_MAPPING: [&str; 7] = [
            "ONT",
            "ONTOLOGY",
            "ONG",
            "ONTOLOGY GAS",
            "ONT GAS",
            "ONTOLOGY FUEL",
            "ONG TOKEN",
        ];
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
        // Beam (новый)
        const BEAM_MAPPING: [&str; 5] = [
            "BEAM",
            "BEAM NETWORK",
            "BEAM CHAIN",
            "BEAM BLOCKCHAIN",
            "BEAM PROTOCOL",
        ];
        for mapped in BEAM_MAPPING {
            m.insert(mapped, NetworkType::Beam);
        }

        // Bitgert (BRISE) (новый)
        const BITGERT_MAPPING: [&str; 6] = [
            "BRISE",
            "BITGERT",
            "BITGERT CHAIN",
            "BITGERT NETWORK",
            "BRISE CHAIN",
            "BRISE NETWORK",
        ];
        for mapped in BITGERT_MAPPING {
            m.insert(mapped, NetworkType::Bitgert);
        }

        // Hype (новый)
        const HYPE_MAPPING: [&str; 6] = [
            "HYPE",
            "HYPE NETWORK",
            "HYPE CHAIN",
            "HYPE BLOCKCHAIN",
            "HYPE PROTOCOL",
            "HYPEEVM",
        ];
        for mapped in HYPE_MAPPING {
            m.insert(mapped, NetworkType::Hype);
        }

        // PIVX (новый)
        const PIVX_MAPPING: [&str; 5] = [
            "PIVX",
            "PRIVATE INSTANT VERIFIED TRANSACTION",
            "PIVX NETWORK",
            "PIVX CHAIN",
            "PIVX BLOCKCHAIN",
        ];
        for mapped in PIVX_MAPPING {
            m.insert(mapped, NetworkType::Pivx);
        }

        // EthereumPoW (ETHW) (новый)
        const ETHW_MAPPING: [&str; 6] = [
            "ETHW",
            "ETHEREUM POW",
            "ETHEREUM PROOF OF WORK",
            "ETHW NETWORK",
            "ETHW CHAIN",
            "ETHEREUMW",
        ];
        for mapped in ETHW_MAPPING {
            m.insert(mapped, NetworkType::EthereumPoW);
        }

        // Oraichain (ORAI) (новый)
        const ORAICHAIN_MAPPING: [&str; 5] = [
            "ORAI",
            "ORAICHAIN",
            "ORAI CHAIN",
            "ORAI NETWORK",
            "ORAI BLOCKCHAIN",
        ];
        for mapped in ORAICHAIN_MAPPING {
            m.insert(mapped, NetworkType::Oraichain);
        }

        // Namada (новый)
        const NAMADA_MAPPING: [&str; 5] = [
            "NAMADA",
            "NAMADA NETWORK",
            "NAMADA CHAIN",
            "NAMADA PROTOCOL",
            "NAMADA BLOCKCHAIN",
        ];
        for mapped in NAMADA_MAPPING {
            m.insert(mapped, NetworkType::Namada);
        }

        // MANTRA Chain (новый)
        const MANTRA_CHAIN_MAPPING: [&str; 7] = [
            "MANTRA",
            "MANTRA CHAIN",
            "MANTRA NETWORK",
            "MANTRA PROTOCOL",
            "MANTRA BLOCKCHAIN",
            "OM",
            "OMNETWORK",
        ];
        for mapped in MANTRA_CHAIN_MAPPING {
            m.insert(mapped, NetworkType::MantraChain);
        }

        // Shibarium (BONE) (новый)
        const SHIBARIUM_MAPPING: [&str; 6] = [
            "BONE",
            "SHIBARIUM",
            "SHIBARIUM NETWORK",
            "SHIBARIUM CHAIN",
            "SHIBARIUM L2",
            "SHIBA INU L2",
        ];
        for mapped in SHIBARIUM_MAPPING {
            m.insert(mapped, NetworkType::Shibarium);
        }

        // Tezos (XTZ) (новый)
        const TEZOS_MAPPING: [&str; 6] = [
            "XTZ",
            "TEZOS",
            "TEZOS NETWORK",
            "TEZOS BLOCKCHAIN",
            "TEZOS PROTOCOL",
            "XTZ BLOCKCHAIN",
        ];
        for mapped in TEZOS_MAPPING {
            m.insert(mapped, NetworkType::Tezos);
        }

        // Etherlink (XTZEVM) - Tezos EVM (новый)
        const ETHERLINK_MAPPING: [&str; 5] = [
            "XTZEVM",
            "ETHERLINK",
            "TEZOS EVM",
            "ETHERLINK NETWORK",
            "TEZOS EVM CHAIN",
        ];
        for mapped in ETHERLINK_MAPPING {
            m.insert(mapped, NetworkType::Etherlink);
        }

        // Zircuit (ZRC) (новый)
        const ZIRCUIT_MAPPING: [&str; 5] = [
            "ZRC",
            "ZIRCUIT",
            "ZIRCUIT NETWORK",
            "ZIRCUIT CHAIN",
            "ZIRCUIT PROTOCOL",
        ];
        for mapped in ZIRCUIT_MAPPING {
            m.insert(mapped, NetworkType::Zircuit);
        }

        // Telos (TLOS) (новый)
        const TELOS_MAPPING: [&str; 7] = [
            "TLOS",
            "TLOSEVM",
            "TELOS",
            "TELOS ZERO",
            "TELOS EVM",
            "TELOS NETWORK",
            "TELOS CHAIN",
        ];
        for mapped in TELOS_MAPPING {
            m.insert(mapped, NetworkType::Telos);
        }

        // Unicorn Ultra Solaris (новый)
        const UNICORN_ULTRA_MAPPING: [&str; 6] = [
            "UNICORNULTRASOLARIS",
            "UNICORN ULTRA SOLARIS",
            "U2U SOLARIS",
            "SOLARIS",
            "UNICORN SOLARIS",
            "U2U NETWORK",
        ];
        for mapped in UNICORN_ULTRA_MAPPING {
            m.insert(mapped, NetworkType::UnicornUltraSolaris);
        }

        // Analog (ANLOG) (новый)
        const ANALOG_MAPPING: [&str; 5] = [
            "ANLOG",
            "ANALOG",
            "ANALOG NETWORK",
            "ANALOG CHAIN",
            "ANALOG PROTOCOL",
        ];
        for mapped in ANALOG_MAPPING {
            m.insert(mapped, NetworkType::Analog);
        }

        // Tap Protocol (TAP) - Bitcoin Ordinals протокол
        const TAP_MAPPING: [&str; 5] = [
            "TAP",
            "TAP PROTOCOL",
            "BITCOIN TAP",
            "TAPROOT",
            "TAP ORDINALS",
        ];
        for mapped in TAP_MAPPING {
            m.insert(mapped, NetworkType::Bitcoin);
        }

        // VANA (новый)
        const VANA_MAPPING: [&str; 5] = [
            "VANA",
            "VANA NETWORK",
            "VANA CHAIN",
            "VANA BLOCKCHAIN",
            "VANA PROTOCOL",
        ];
        for mapped in VANA_MAPPING {
            m.insert(mapped, NetworkType::Vana);
        }

        // Swell (новый)
        const SWELL_MAPPING: [&str; 5] = [
            "SWELL",
            "SWELL NETWORK",
            "SWELL CHAIN",
            "SWELL PROTOCOL",
            "SWELL LIQUID STAKING",
        ];
        for mapped in SWELL_MAPPING {
            m.insert(mapped, NetworkType::Swell);
        }

        // Aleo (новый)
        const ALEO_MAPPING: [&str; 5] = [
            "ALEO",
            "ALEO NETWORK",
            "ALEO CHAIN",
            "ALEO BLOCKCHAIN",
            "ALEO PROTOCOL",
        ];
        for mapped in ALEO_MAPPING {
            m.insert(mapped, NetworkType::Aleo);
        }

        // Flare (новый)
        const FLARE_MAPPING: [&str; 6] = [
            "FLARE",
            "FLR",
            "FLARE NETWORK",
            "FLARE CHAIN",
            "FLARE BLOCKCHAIN",
            "FLARE PROTOCOL",
        ];
        for mapped in FLARE_MAPPING {
            m.insert(mapped, NetworkType::Flare);
        }

        // Venom (новый)
        const VENOM_MAPPING: [&str; 6] = [
            "VENOM",
            "VENOM BLOCKCHAIN",
            "VENOM NETWORK",
            "VENOM CHAIN",
            "VENOM PROTOCOL",
            "VENOM FOUNDATION",
        ];
        for mapped in VENOM_MAPPING {
            m.insert(mapped, NetworkType::Venom);
        }

        // BTS (Bitshares) (новый)
        const BITSHARES_MAPPING: [&str; 6] = [
            "BTS",
            "BITSHARES",
            "BTS NETWORK",
            "BITSHARES NETWORK",
            "BTS CHAIN",
            "BITSHARES CHAIN",
        ];
        for mapped in BITSHARES_MAPPING {
            m.insert(mapped, NetworkType::Bitshares);
        }

        // Hydragon (новый)
        const HYDRAGON_MAPPING: [&str; 5] = [
            "HYDRAGON",
            "HYDRAGON NETWORK",
            "HYDRAGON CHAIN",
            "HYDRAGON BLOCKCHAIN",
            "HYDRAGON PROTOCOL",
        ];
        for mapped in HYDRAGON_MAPPING {
            m.insert(mapped, NetworkType::Hydragon);
        }

        // Sonic (новый)
        const SONIC_MAPPING: [&str; 6] = [
            "SONIC",
            "SONIC NETWORK",
            "SONIC CHAIN",
            "SONIC BLOCKCHAIN",
            "SONIC PROTOCOL",
            "SONIC L2",
        ];
        for mapped in SONIC_MAPPING {
            m.insert(mapped, NetworkType::Sonic);
        }

        // NEW (New Chain) (новый)
        const NEW_CHAIN_MAPPING: [&str; 5] = [
            "NEW",
            "NEW CHAIN",
            "NEW NETWORK",
            "NEW BLOCKCHAIN",
            "NEW PROTOCOL",
        ];
        for mapped in NEW_CHAIN_MAPPING {
            m.insert(mapped, NetworkType::NewChain);
        }

        // Casper (CSPR) (новый)
        const CASPER_MAPPING: [&str; 6] = [
            "CSPR",
            "CASPER",
            "CASPER NETWORK",
            "CASPER CHAIN",
            "CASPER BLOCKCHAIN",
            "CASPER PROTOCOL",
        ];
        for mapped in CASPER_MAPPING {
            m.insert(mapped, NetworkType::Casper);
        }

        // Hive (новый)
        const HIVE_MAPPING: [&str; 6] = [
            "HIVE",
            "HIVE NETWORK",
            "HIVE CHAIN",
            "HIVE BLOCKCHAIN",
            "HIVE PROTOCOL",
            "HIVE ENGINE",
        ];
        for mapped in HIVE_MAPPING {
            m.insert(mapped, NetworkType::Hive);
        }

        // Octa (новый)
        const OCTA_MAPPING: [&str; 5] = [
            "OCTA",
            "OCTA NETWORK",
            "OCTA CHAIN",
            "OCTA BLOCKCHAIN",
            "OCTA PROTOCOL",
        ];
        for mapped in OCTA_MAPPING {
            m.insert(mapped, NetworkType::Octa);
        }

        // Shentu (CTK) (новый)
        const SHENTU_MAPPING: [&str; 6] = [
            "CTK",
            "SHENTU",
            "SHENTU NETWORK",
            "SHENTU CHAIN",
            "SHENTU PROTOCOL",
            "CERTIK CHAIN",
        ];
        for mapped in SHENTU_MAPPING {
            m.insert(mapped, NetworkType::Shentu);
        }

        // LUKSO (LYX) (новый)
        const LUKSO_MAPPING: [&str; 6] = [
            "LYX",
            "LUKSO",
            "LUKSO NETWORK",
            "LUKSO CHAIN",
            "LUKSO BLOCKCHAIN",
            "LUKSO PROTOCOL",
        ];
        for mapped in LUKSO_MAPPING {
            m.insert(mapped, NetworkType::Lukso);
        }

        // Nibiru (NIBI) (новый)
        const NIBIRU_MAPPING: [&str; 6] = [
            "NIBI",
            "NIBIRU",
            "NIBIRU NETWORK",
            "NIBIRU CHAIN",
            "NIBIRU PROTOCOL",
            "NIBIRU BLOCKCHAIN",
        ];
        for mapped in NIBIRU_MAPPING {
            m.insert(mapped, NetworkType::Nibiru);
        }

        // Osmosis (OSMO) (новый)
        const OSMOSIS_MAPPING: [&str; 6] = [
            "OSMO",
            "OSMOSIS",
            "OSMOSIS NETWORK",
            "OSMOSIS CHAIN",
            "OSMOSIS PROTOCOL",
            "OSMOSIS ZONE",
        ];
        for mapped in OSMOSIS_MAPPING {
            m.insert(mapped, NetworkType::Osmosis);
        }

        // TBC (The Bits Chain?) (новый)
        const TBC_MAPPING: [&str; 5] = [
            "TBC",
            "TBC NETWORK",
            "TBC CHAIN",
            "TBC BLOCKCHAIN",
            "TBC PROTOCOL",
        ];
        for mapped in TBC_MAPPING {
            m.insert(mapped, NetworkType::Tbc);
        }

        // Electroneum (ETN) (новый)
        const ELECTRONEUM_MAPPING: [&str; 6] = [
            "ETN",
            "ELECTRONEUM",
            "ELECTRONEUM NETWORK",
            "ELECTRONEUM CHAIN",
            "ELECTRONEUM BLOCKCHAIN",
            "ETN NETWORK",
        ];
        for mapped in ELECTRONEUM_MAPPING {
            m.insert(mapped, NetworkType::Electroneum);
        }

        // Iron (IRON) (новый)
        const IRON_MAPPING: [&str; 5] = [
            "IRON",
            "IRON NETWORK",
            "IRON CHAIN",
            "IRON BLOCKCHAIN",
            "IRON PROTOCOL",
        ];
        for mapped in IRON_MAPPING {
            m.insert(mapped, NetworkType::Iron);
        }

        // FIO Protocol (новый)
        const FIO_MAPPING: [&str; 6] = [
            "FIO",
            "FIO PROTOCOL",
            "FIO NETWORK",
            "FIO CHAIN",
            "FIO BLOCKCHAIN",
            "FOUNDATION FOR INTERWALLET OPERABILITY",
        ];
        for mapped in FIO_MAPPING {
            m.insert(mapped, NetworkType::Fio);
        }

        // Radix (XRD) (новый)
        const RADIX_MAPPING: [&str; 7] = [
            "XRD",
            "RADIX",
            "RADIX NETWORK",
            "RADIX CHAIN",
            "RADIX BLOCKCHAIN",
            "RADIX PROTOCOL",
            "RADIX LEDGER",
        ];
        for mapped in RADIX_MAPPING {
            m.insert(mapped, NetworkType::Radix);
        }

        // Cronos (CRO) (новый)
        const CRONOS_MAPPING: [&str; 7] = [
            "CRO",
            "CRONOS",
            "CRONOS NETWORK",
            "CRONOS CHAIN",
            "CRONOS BLOCKCHAIN",
            "CRONOS PROTOCOL",
            "CRYPTO ORG CHAIN",
        ];
        for mapped in CRONOS_MAPPING {
            m.insert(mapped, NetworkType::Cronos);
        }

        // XNA (Neurai) (новый)
        const XNA_MAPPING: [&str; 6] = [
            "XNA",
            "NEURAI",
            "NEURAI NETWORK",
            "NEURAI CHAIN",
            "NEURAI BLOCKCHAIN",
            "NEURAI PROTOCOL",
        ];
        for mapped in XNA_MAPPING {
            m.insert(mapped, NetworkType::Xna);
        }

        // Fractal (fb) (новый)
        const FRACTAL_MAPPING: [&str; 6] = [
            "FB",
            "FRACTAL",
            "FRACTAL NETWORK",
            "FRACTAL CHAIN",
            "FRACTAL BLOCKCHAIN",
            "FRACTAL PROTOCOL",
        ];
        for mapped in FRACTAL_MAPPING {
            m.insert(mapped, NetworkType::Fractal);
        }

        // Ergo (новый)
        const ERGO_MAPPING: [&str; 7] = [
            "ERGO",
            "ERG",
            "ERGO NETWORK",
            "ERGO CHAIN",
            "ERGO BLOCKCHAIN",
            "ERGO PLATFORM",
            "ERGO PROTOCOL",
        ];
        for mapped in ERGO_MAPPING {
            m.insert(mapped, NetworkType::Ergo);
        }

        // Fuse (новый)
        const FUSE_MAPPING: [&str; 6] = [
            "FUSE",
            "FUSE NETWORK",
            "FUSE CHAIN",
            "FUSE BLOCKCHAIN",
            "FUSE PROTOCOL",
            "FUSE CASH",
        ];
        for mapped in FUSE_MAPPING {
            m.insert(mapped, NetworkType::Fuse);
        }

        // Vaulta (EOS?) (новый)
        const VAULTA_MAPPING: [&str; 6] = [
            "EOS",
            "VAULTA",
            "VAULTA NETWORK",
            "VAULTA CHAIN",
            "VAULTA BLOCKCHAIN",
            "VAULTA PROTOCOL",
        ];
        for mapped in VAULTA_MAPPING {
            m.insert(mapped, NetworkType::Vaulta);
        }

        // Shardeum (новый)
        const SHARDEUM_MAPPING: [&str; 7] = [
            "SHARDEUM",
            "SHARDEUM NETWORK",
            "SHARDEUM CHAIN",
            "SHARDEUM BLOCKCHAIN",
            "SHARDEUM PROTOCOL",
            "SHARDEUM L1",
            "SHM",
        ];
        for mapped in SHARDEUM_MAPPING {
            m.insert(mapped, NetworkType::Shardeum);
        }

        // HashKey Chain (HSK) (новый)
        const HASHKEY_MAPPING: [&str; 5] = [
            "HSK",
            "HASHKEY",
            "HASHKEY CHAIN",
            "HASHKEY NETWORK",
            "HASHKEY BLOCKCHAIN",
        ];
        for mapped in HASHKEY_MAPPING {
            m.insert(mapped, NetworkType::HashKey);
        }

        // StaFi (FIS) (новый)
        const STAFI_MAPPING: [&str; 5] = [
            "FIS",
            "STAFI",
            "STAFI PROTOCOL",
            "STAFI NETWORK",
            "STAFI CHAIN",
        ];
        for mapped in STAFI_MAPPING {
            m.insert(mapped, NetworkType::StaFi);
        }

        // Peaq (PEAQ) (новый)
        const PEAQ_MAPPING: [&str; 6] = [
            "PEAQ",
            "PEAQEVM",
            "PEAQ EVM",
            "PEAQ NETWORK",
            "PEAQ CHAIN",
            "PEAQ PROTOCOL",
        ];
        for mapped in PEAQ_MAPPING {
            m.insert(mapped, NetworkType::Peaq);
        }

        // Steem (новый)
        const STEEM_MAPPING: [&str; 5] = [
            "STEEM",
            "STEEM NETWORK",
            "STEEM CHAIN",
            "STEEM BLOCKCHAIN",
            "STEEM PROTOCOL",
        ];
        for mapped in STEEM_MAPPING {
            m.insert(mapped, NetworkType::Steem);
        }

        // Sophon (SOPH) (новый)
        const SOPHON_MAPPING: [&str; 5] = [
            "SOPH",
            "SOPHON",
            "SOPHON NETWORK",
            "SOPHON CHAIN",
            "SOPHON PROTOCOL",
        ];
        for mapped in SOPHON_MAPPING {
            m.insert(mapped, NetworkType::Sophon);
        }

        // Polymesh (POLYX) (новый)
        const POLYMESH_MAPPING: [&str; 6] = [
            "POLYX",
            "POLYMESH",
            "POLYMESH NETWORK",
            "POLYMESH CHAIN",
            "POLYMESH BLOCKCHAIN",
            "POLYMESH PROTOCOL",
        ];
        for mapped in POLYMESH_MAPPING {
            m.insert(mapped, NetworkType::Polymesh);
        }

        // Numbers Protocol (NUM) (новый)
        const NUMBERS_MAPPING: [&str; 6] = [
            "NUM",
            "NUMBERS",
            "NUMBERS PROTOCOL",
            "NUMBERS NETWORK",
            "NUMBERS CHAIN",
            "NUMBERS BLOCKCHAIN",
        ];
        for mapped in NUMBERS_MAPPING {
            m.insert(mapped, NetworkType::Numbers);
        }

        // Kaspa (KAS) (новый)
        const KASPA_MAPPING: [&str; 5] = [
            "KAS",
            "KASPA",
            "KASPA NETWORK",
            "KASPA CHAIN",
            "KASPA BLOCKCHAIN",
        ];
        for mapped in KASPA_MAPPING {
            m.insert(mapped, NetworkType::Kaspa);
        }

        // Constellation (DAG) (новый)
        const CONSTELLATION_MAPPING: [&str; 6] = [
            "DAG",
            "CONSTELLATION",
            "CONSTELLATION NETWORK",
            "CONSTELLATION CHAIN",
            "CONSTELLATION BLOCKCHAIN",
            "CONSTELLATION PROTOCOL",
        ];
        for mapped in CONSTELLATION_MAPPING {
            m.insert(mapped, NetworkType::Constellation);
        }

        // t3rn (T3RN) (новый)
        const T3RN_MAPPING: [&str; 5] = [
            "T3RN",
            "T3RN NETWORK",
            "T3RN CHAIN",
            "T3RN PROTOCOL",
            "T3RN BLOCKCHAIN",
        ];
        for mapped in T3RN_MAPPING {
            m.insert(mapped, NetworkType::T3rn);
        }

        // Bitcoin SV (BSV) (новый) - ОТДЕЛЬНАЯ СЕТЬ ОТ BITCOIN!
        const BITCOIN_SV_MAPPING: [&str; 6] = [
            "BSV",
            "BITCOIN SV",
            "BITCOIN SATOSHI VISION",
            "BITCOINSV",
            "BSV NETWORK",
            "BSV CHAIN",
        ];
        for mapped in BITCOIN_SV_MAPPING {
            m.insert(mapped, NetworkType::BitcoinSV);
        }

        // Maverick Protocol (MVRK) (новый)
        const MAVERICK_MAPPING: [&str; 6] = [
            "MVRK",
            "MAVERICK",
            "MAVERICK PROTOCOL",
            "MAVERICK NETWORK",
            "MAVERICK CHAIN",
            "MAVERICK BLOCKCHAIN",
        ];
        for mapped in MAVERICK_MAPPING {
            m.insert(mapped, NetworkType::Maverick);
        }

        // Monero (XMR) (новый)
        const MONERO_MAPPING: [&str; 5] = [
            "XMR",
            "MONERO",
            "MONERO NETWORK",
            "MONERO CHAIN",
            "MONERO BLOCKCHAIN",
        ];
        for mapped in MONERO_MAPPING {
            m.insert(mapped, NetworkType::Monero);
        }

        // MON (Monarch?) (новый)
        const MON_MAPPING: [&str; 5] = [
            "MON",
            "MONARCH",
            "MON NETWORK",
            "MON CHAIN",
            "MON BLOCKCHAIN",
        ];
        for mapped in MON_MAPPING {
            m.insert(mapped, NetworkType::Mon);
        }

        // Ultima (новый)
        const ULTIMA_MAPPING: [&str; 5] = [
            "ULTIMA",
            "ULTIMA NETWORK",
            "ULTIMA CHAIN",
            "ULTIMA BLOCKCHAIN",
            "ULTIMA PROTOCOL",
        ];
        for mapped in ULTIMA_MAPPING {
            m.insert(mapped, NetworkType::Ultima);
        }

        // GraphLinq (GLQ) (новый)
        const GRAPHLINQ_MAPPING: [&str; 6] = [
            "GLQ",
            "GRAPHLINQ",
            "GRAPHLINQ CHAIN",
            "GRAPHLINQ NETWORK",
            "GRAPHLINQ PROTOCOL",
            "GRAPHLINQ BLOCKCHAIN",
        ];
        for mapped in GRAPHLINQ_MAPPING {
            m.insert(mapped, NetworkType::GraphLinq);
        }

        // IRISnet (IRIS) (новый)
        const IRISNET_MAPPING: [&str; 6] = [
            "IRIS",
            "IRISNET",
            "IRIS NETWORK",
            "IRISNET CHAIN",
            "IRISNET PROTOCOL",
            "IRISNET BLOCKCHAIN",
        ];
        for mapped in IRISNET_MAPPING {
            m.insert(mapped, NetworkType::Irisnet);
        }

        // Stratis (STRAX) (новый)
        const STRATIS_MAPPING: [&str; 8] = [
            "STRAX",
            "STRATIS",
            "STRATIS EVM",
            "STRATIS NETWORK",
            "STRATIS CHAIN",
            "STRATIS BLOCKCHAIN",
            "STRATIS PROTOCOL",
            "STRATISEVM",
        ];
        for mapped in STRATIS_MAPPING {
            m.insert(mapped, NetworkType::Stratis);
        }

        // Pocket Network (POKT) (новый)
        const POCKET_MAPPING: [&str; 7] = [
            "POKT",
            "POCKET",
            "POCKET NETWORK",
            "POCKET CHAIN",
            "POCKET BLOCKCHAIN",
            "POCKET PROTOCOL",
            "POCKET NETWORK PROTOCOL",
        ];
        for mapped in POCKET_MAPPING {
            m.insert(mapped, NetworkType::Pocket);
        }

        // aelf (ELF) (новый)
        const AELF_MAPPING: [&str; 6] = [
            "ELF",
            "AELF",
            "AELF NETWORK",
            "AELF CHAIN",
            "AELF BLOCKCHAIN",
            "AELF PROTOCOL",
        ];
        for mapped in AELF_MAPPING {
            m.insert(mapped, NetworkType::Aelf);
        }

        // Ark (ARK) (новый)
        const ARK_MAPPING: [&str; 5] = [
            "ARK",
            "ARK NETWORK",
            "ARK CHAIN",
            "ARK BLOCKCHAIN",
            "ARK PROTOCOL",
        ];
        for mapped in ARK_MAPPING {
            m.insert(mapped, NetworkType::Ark);
        }

        // Akash (AKT) (новый)
        const AKASH_MAPPING: [&str; 6] = [
            "AKT",
            "AKASH",
            "AKASH NETWORK",
            "AKASH CHAIN",
            "AKASH BLOCKCHAIN",
            "AKASH PROTOCOL",
        ];
        for mapped in AKASH_MAPPING {
            m.insert(mapped, NetworkType::Akash);
        }

        // Core (CORE) (новый)
        const CORE_MAPPING: [&str; 6] = [
            "CORE",
            "CORE NETWORK",
            "CORE CHAIN",
            "CORE BLOCKCHAIN",
            "CORE DAO",
            "CORE PROTOCOL",
        ];
        for mapped in CORE_MAPPING {
            m.insert(mapped, NetworkType::Core);
        }

        // X Layer (новый)
        const XLAYER_MAPPING: [&str; 6] = [
            "XLAYER",
            "X LAYER",
            "XLAYER NETWORK",
            "XLAYER CHAIN",
            "XLAYER BLOCKCHAIN",
            "XLAYER PROTOCOL",
        ];
        for mapped in XLAYER_MAPPING {
            m.insert(mapped, NetworkType::XLayer);
        }

        // Allora (ALLORA/ALLO) (новый)
        const ALLORA_MAPPING: [&str; 6] = [
            "ALLORA",
            "ALLO",
            "ALLORA NETWORK",
            "ALLORA CHAIN",
            "ALLORA BLOCKCHAIN",
            "ALLORA PROTOCOL",
        ];
        for mapped in ALLORA_MAPPING {
            m.insert(mapped, NetworkType::Allora);
        }

        // Emyria (EMYC) (новый)
        const EMYRIA_MAPPING: [&str; 5] = [
            "EMYC",
            "EMYRIA",
            "EMYRIA NETWORK",
            "EMYRIA CHAIN",
            "EMYRIA BLOCKCHAIN",
        ];
        for mapped in EMYRIA_MAPPING {
            m.insert(mapped, NetworkType::Emyria);
        }

        // Ethereum Classic (ETC) (новый) - ОТДЕЛЬНАЯ СЕТЬ ОТ ETHEREUM!
        const ETHEREUM_CLASSIC_MAPPING: [&str; 7] = [
            "ETC",
            "ETHEREUM CLASSIC",
            "ETHEREUMCLASSIC",
            "ETC NETWORK",
            "ETC CHAIN",
            "ETC BLOCKCHAIN",
            "ETH CLASSIC",
        ];
        for mapped in ETHEREUM_CLASSIC_MAPPING {
            m.insert(mapped, NetworkType::EthereumClassic);
        }

        // Proton (XPR) (новый)
        const PROTON_MAPPING: [&str; 6] = [
            "XPR",
            "PROTON",
            "PROTON NETWORK",
            "PROTON CHAIN",
            "PROTON BLOCKCHAIN",
            "PROTON PROTOCOL",
        ];
        for mapped in PROTON_MAPPING {
            m.insert(mapped, NetworkType::Proton);
        }

        const CAMP_CHAIN_MAPPING: [&str; 6] = [
            "CAMP",
            "CAMPCHAIN",
            "CAMP CHAIN",
            "CAMP NETWORK",
            "CAMP BLOCKCHAIN",
            "CAMP PROTOCOL",
        ];
        for mapped in CAMP_CHAIN_MAPPING {
            m.insert(mapped, NetworkType::Camp);
        }

        // Avail (AVL) (новый)
        const AVAIL_MAPPING: [&str; 6] = [
            "AVAIL",
            "AVL",
            "AVAIL NETWORK",
            "AVAIL CHAIN",
            "AVAIL BLOCKCHAIN",
            "AVAIL PROTOCOL",
        ];
        for mapped in AVAIL_MAPPING {
            m.insert(mapped, NetworkType::Avail);
        }

        // KUB Chain (новый)
        const KUB_MAPPING: [&str; 5] = [
            "KUB",
            "KUB CHAIN",
            "KUB NETWORK",
            "KUB BLOCKCHAIN",
            "KUB PROTOCOL",
        ];
        for mapped in KUB_MAPPING {
            m.insert(mapped, NetworkType::Kub);
        }

        // AB Core (AB) (новый)
        const ABCORE_MAPPING: [&str; 6] = [
            "AB",
            "ABCORE",
            "AB CORE",
            "AB NETWORK",
            "AB CHAIN",
            "AB BLOCKCHAIN",
        ];
        for mapped in ABCORE_MAPPING {
            m.insert(mapped, NetworkType::ABCore);
        }

        // Humanode (HMND) (новый)
        const HUMANODE_MAPPING: [&str; 6] = [
            "HMND",
            "HUMANODE",
            "HUMANODE NETWORK",
            "HUMANODE CHAIN",
            "HUMANODE BLOCKCHAIN",
            "HUMANODE PROTOCOL",
        ];
        for mapped in HUMANODE_MAPPING {
            m.insert(mapped, NetworkType::Humanode);
        }

        // Dynex (DNX) (новый)
        const DYNEX_MAPPING: [&str; 5] = [
            "DNX",
            "DYNEX",
            "DYNEX NETWORK",
            "DYNEX CHAIN",
            "DYNEX BLOCKCHAIN",
        ];
        for mapped in DYNEX_MAPPING {
            m.insert(mapped, NetworkType::Dynex);
        }

        // Saga (SAGA) (новый)
        const SAGA_MAPPING: [&str; 5] = [
            "SAGA",
            "SAGA NETWORK",
            "SAGA CHAIN",
            "SAGA BLOCKCHAIN",
            "SAGA PROTOCOL",
        ];
        for mapped in SAGA_MAPPING {
            m.insert(mapped, NetworkType::Saga);
        }

        // Oasys (OAS) (новый)
        const OASYS_MAPPING: [&str; 6] = [
            "OAS",
            "OASYS",
            "OASYS NETWORK",
            "OASYS CHAIN",
            "OASYS BLOCKCHAIN",
            "OASYS PROTOCOL",
        ];
        for mapped in OASYS_MAPPING {
            m.insert(mapped, NetworkType::Oasys);
        }

        // Wanchain (WAN) (новый)
        const WANCHAIN_MAPPING: [&str; 6] = [
            "WAN",
            "WANCHAIN",
            "WAN CHAIN",
            "WAN NETWORK",
            "WAN BLOCKCHAIN",
            "WAN PROTOCOL",
        ];
        for mapped in WANCHAIN_MAPPING {
            m.insert(mapped, NetworkType::Wanchain);
        }

        // Zelcash (ZEL) (новый)
        const ZELCASH_MAPPING: [&str; 6] = [
            "ZEL",
            "ZELCASH",
            "ZEL CASH",
            "ZEL NETWORK",
            "ZEL CHAIN",
            "ZEL BLOCKCHAIN",
        ];
        for mapped in ZELCASH_MAPPING {
            m.insert(mapped, NetworkType::Zelcash);
        }

        // Luckycoin (новый)
        const LUCKYCOIN_MAPPING: [&str; 5] = [
            "LUCKYCOIN",
            "LUCKY COIN",
            "LUCKY NETWORK",
            "LUCKY CHAIN",
            "LUCKY BLOCKCHAIN",
        ];
        for mapped in LUCKYCOIN_MAPPING {
            m.insert(mapped, NetworkType::Luckycoin);
        }

        // Lamina1 (новый)
        const LAMINA1_MAPPING: [&str; 5] = [
            "LAMINA1",
            "LAMINA 1",
            "LAMINA ONE",
            "LAMINA1 NETWORK",
            "LAMINA1 CHAIN",
        ];
        for mapped in LAMINA1_MAPPING {
            m.insert(mapped, NetworkType::Lamina1);
        }

        // ZkVerify (новый)
        const ZKVERIFY_MAPPING: [&str; 6] = [
            "ZKVERIFY",
            "ZK VERIFY",
            "ZKVERIFY NETWORK",
            "ZKVERIFY CHAIN",
            "ZKVERIFY PROTOCOL",
            "ZKVERIFY BLOCKCHAIN",
        ];
        for mapped in ZKVERIFY_MAPPING {
            m.insert(mapped, NetworkType::ZkVerify);
        }

        // Nervos Network (CKB) (новый)
        const NERVOS_MAPPING: [&str; 7] = [
            "CKB",
            "NERVOS",
            "NERVOS NETWORK",
            "NERVOS CHAIN",
            "NERVOS BLOCKCHAIN",
            "NERVOS PROTOCOL",
            "COMMON KNOWLEDGE BASE",
        ];
        for mapped in NERVOS_MAPPING {
            m.insert(mapped, NetworkType::Nervos);
        }

        // Blast (BLAST) (новый)
        const BLAST_MAPPING: [&str; 6] = [
            "BLAST",
            "BLAST NETWORK",
            "BLAST CHAIN",
            "BLAST BLOCKCHAIN",
            "BLAST PROTOCOL",
            "BLAST L2",
        ];
        for mapped in BLAST_MAPPING {
            m.insert(mapped, NetworkType::Blast);
        }

        // Aleph Zero (AZERO) (новый)
        const ALEPH_ZERO_MAPPING: [&str; 7] = [
            "AZERO",
            "ALEPH ZERO",
            "ALEPHZERO",
            "ALEPH ZERO NETWORK",
            "ALEPH ZERO CHAIN",
            "ALEPH ZERO BLOCKCHAIN",
            "ALEPH ZERO PROTOCOL",
        ];
        for mapped in ALEPH_ZERO_MAPPING {
            m.insert(mapped, NetworkType::AlephZero);
        }

        // Somnia (SOMI) (новый)
        const SOMNIA_MAPPING: [&str; 6] = [
            "SOMI",
            "SOMNIA",
            "SOMNIA NETWORK",
            "SOMNIA CHAIN",
            "SOMNIA BLOCKCHAIN",
            "SOMNIA PROTOCOL",
        ];
        for mapped in SOMNIA_MAPPING {
            m.insert(mapped, NetworkType::Somnia);
        }

        m
    })
}
