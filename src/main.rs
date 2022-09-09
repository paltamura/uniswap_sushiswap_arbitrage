pub mod constants;
pub mod ethereum_interaction;
pub mod models;
pub mod online_analyzer;
pub mod pre_process;
pub mod utils;
use bigdecimal::num_bigint::BigInt;
use constants::*;
use online_analyzer::*;
use pre_process::*;
use std::time::Instant;

fn main() {
    if PRE_PROCESS {
        pre_processing().expect("error pre-processing data");
    }
    let mut online_analyzer = OnlineAnalyzer::new();
    online_analyzer.init().expect("error initializing OnlineAnalyzer data");
    let before = Instant::now();
    online_analyzer
        .find_profit_chances(&BigInt::from(AMOUNT_OF_SIMULATION_IN_WEI))
        .expect("error looking for profit chances");
    println!("find_profit_chances -> elapsed time: {:.2?}", before.elapsed());
}
