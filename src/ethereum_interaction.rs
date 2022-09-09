use crate::constants;
use bigdecimal::num_bigint::{BigInt, Sign};
use constants::*;
use ethers_contract::Contract;
use ethers_core::abi::Detokenize;
use ethers_core::{
    abi::Abi,
    types::{Address, U128},
};
use ethers_providers::{Http, Middleware, Provider};
use std::{convert::TryFrom, fs};

#[tokio::main]
pub async fn read_pair_info(token_address: &str) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
    let pair_contract = load_contract(token_address, UNISWAP_PAIR_ABI_FILE)?;
    Ok((
        invoke_contract_method::<_, String>(&pair_contract, "name").await?,
        invoke_contract_method::<_, String>(&pair_contract, "symbol").await?,
        format!("{:?}", invoke_contract_method::<_, Address>(&pair_contract, "token0").await?),
        format!("{:?}", invoke_contract_method::<_, Address>(&pair_contract, "token1").await?),
    ))
}

#[tokio::main]
pub async fn read_reserves(pair_string_addresses: &Vec<String>) -> Result<Vec<BigInt>, Box<dyn std::error::Error>> {
    match load_contract(UNISWAP_VIEW_ADDRESS, UNISWAP_VIEW_ABI_FILE) {
        Ok(uniswap_view_contract) => {
            let liquidity_pairs: Vec<Address> = pair_string_addresses
                .into_iter()
                .filter(|x| x.parse::<Address>().is_ok())
                .map(|x| x.parse::<Address>().unwrap())
                .collect();

            Ok(uniswap_view_contract
                .method::<_, Vec<U128>>("viewPair", liquidity_pairs)?
                .call()
                .await?
                .into_iter()
                .map(|x| BigInt::from_bytes_be(Sign::Plus, &x.as_u128().to_be_bytes()))
                .collect())
        },
        Err(err) => Err(err),
    }
}

pub fn load_contract(contract_address: &str, contract_abi_file: &str) -> Result<Contract<Provider<Http>>, Box<dyn std::error::Error>> {
    let address = contract_address.parse::<Address>()?;
    let abi_str = fs::read_to_string(&format!("{}{}", ABIS_FOLDER, contract_abi_file))?;
    let abi: Abi = serde_json::from_str(&abi_str)?;
    let provider = Provider::<Http>::try_from(RPC_URL_MAINNET)?;
    let contract = Contract::new(address, abi, provider);
    Ok(contract)
}

pub async fn invoke_contract_method<M: Middleware, T: Detokenize>(pair_contract: &Contract<M>, method_name: &str) -> Result<T, Box<dyn std::error::Error>> {
    Ok(pair_contract
        .method::<_, T>(method_name, ())?
        .call()
        .await
        .expect("error reading from blockchain"))
}
