use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonToken {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonNode {
    pub token_out: String,
    pub pair_providers_addresses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct PathImbalance {
    pub path: Path,
    pub in_wei: i128,
    pub out_wei: i128,
    pub imbalance_in_wei: i128,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct EfectivePair {
    pub token_in: String,
    pub token_out: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Path {
    // pub id: usize,
    pub efective_pairs: Vec<EfectivePair>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Pair {
    pub internal_index: usize,
    pub name: String,
    pub symbol: String,
    pub token_0: String,
    pub token_1: String,
    pub address: String,
}
