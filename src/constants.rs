pub const DEBUG: bool = false;
pub const PATH_INDEX_FOR_DEBUG: usize = 0;

pub const PRE_PROCESS: bool = true;
pub const UPDATE_PAIRS: bool = false; // very slow process
pub const FIX_TOKENS: bool = false;

pub const MINIMUM_RESERVES: u64 = 1_000_000_000;
pub const FILTER_SYMMETRIC: bool = true;
pub const INCLUDE_IMPACT_OF_TRANSACTION: bool = true;
pub const AMOUNT_OF_SIMULATION_IN_WEI: u64 = 1_000_000_000_000_000_000;
//pub const AMOUNT_OF_SIMULATION_IN_WEI: u64 = 1_000;

pub const WAD: u64 =           1_000_000_000_000_000_000;
pub const COMMISSION_IN_WAD: u64 = 3_000_000_000_000_000;

pub const RPC_URL_MAINNET: &str = "https://mainnet.infura.io/v3/79408f3788cd4635b40bdd9e4fceaad5";
pub const WETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const UNISWAP_VIEW_ADDRESS: &str = "0x416355755f32b2710ce38725ed0fa102ce7d07e6";
pub const UNISWAP_VIEW_ABI_FILE: &str = "UniswapViewAbi.json";
pub const UNISWAP_PAIR_ABI_FILE: &str = "UniswapPairAbi.json";

pub const DATA_FOLDER: &str = "./data/";
pub const ABIS_FOLDER: &str = "./data/abis/";
pub const WORKING_FILES_FOLDER: &str = "working_files/";
pub const UNI_SUSHI_PATHS_FILE: &str = "uni_sushi_paths.json";
pub const TOKENS_FILE: &str = "tokens.json";
pub const SELECTED_PATHS_FILE: &str = "selected_paths.json";
pub const SELECTED_PAIR_ADDRESSES_FILE: &str = "selected_pair_addresses.json";
pub const ALL_PAIRS_FILE: &str = "all_pairs.json";
pub const IMBALANCES_FILE: &str = "imbalances.json";
