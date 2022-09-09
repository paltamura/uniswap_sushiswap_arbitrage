use crate::{constants::*, ethereum_interaction, models, utils};
use bigdecimal::num_bigint::BigInt;
use ethereum_interaction::*;
use models::*;
use std::{collections::HashMap, time::Instant};
use utils::*;

pub struct OnlineAnalyzer {
    pair_string_addresses: Vec<String>,
    paths: Vec<Path>,
    address_pairs: HashMap<String, Pair>,
}

impl OnlineAnalyzer {
    pub fn new() -> Self {
        OnlineAnalyzer {
            pair_string_addresses: vec![],
            paths: vec![],
            address_pairs: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // pair_string_addresses
        match read_json(&format!("{}{}", WORKING_FILES_FOLDER, SELECTED_PAIR_ADDRESSES_FILE)) {
            Ok(pair_string_addresses) => self.pair_string_addresses = pair_string_addresses,
            Err(err) => return Err(err),
        }

        // paths
        match read_json(&format!("{}{}", WORKING_FILES_FOLDER, SELECTED_PATHS_FILE)) {
            Ok(paths) => self.paths = paths,
            Err(err) => return Err(err),
        }

        // address_pairs
        match read_json::<Vec<Pair>>(&format!("{}{}", WORKING_FILES_FOLDER, ALL_PAIRS_FILE)) {
            Ok(pairs) => {
                for pair in pairs {
                    self.address_pairs.insert(pair.address.clone(), pair.clone());
                }
            },
            Err(err) => return Err(err),
        }

        Ok(())
    }

    pub fn find_profit_chances(&mut self, amount_in_wei: &BigInt) -> Result<(), Box<dyn std::error::Error>> {
        let before = Instant::now();
        let reserves: Vec<BigInt> = read_reserves(&self.pair_string_addresses)?;
        println!("read_reserves -> elapsed time: {:.2?}", before.elapsed());

        let in_wei_i128 = i128::try_from(amount_in_wei.clone()).unwrap();
        let mut imbalances: Vec<PathImbalance> = vec![];
        let mut i = 0;

        for path in &self.paths {
            if !DEBUG || i == PATH_INDEX_FOR_DEBUG {
                if DEBUG {
                    println!("path > {:?}", path);
                }
                let value = self.get_path_cost_in_wei(&reserves, &path, amount_in_wei);
                match value {
                    Some(v) => {
                        let out_wei_i128 = i128::try_from(v.clone()).unwrap();
                        let path_imbalance = PathImbalance {
                            path: path.clone(),
                            in_wei: in_wei_i128,
                            out_wei: out_wei_i128,
                            imbalance_in_wei: out_wei_i128 - in_wei_i128,
                        };
                        imbalances.push(path_imbalance);
                    },
                    None => {},
                };
            }
            i += 1;
        }
        println!("{} paths analyzed", &self.paths.len());
        let before = Instant::now();
        save_json::<Vec<PathImbalance>>(&imbalances, IMBALANCES_FILE).unwrap();
        println!("save_json -> elapsed time: {:.2?}", before.elapsed());
        Ok(())
    }

    fn get_return_payment_from_path(&self, reserves: &Vec<BigInt>, path: &Path, payment: &BigInt) -> Option<BigInt> {
        let mut payment: BigInt = payment.clone();
        for efective_pair in &path.efective_pairs {
            let opt_payment = self.get_return_payment_from_efective_pair(reserves, efective_pair, &payment);
            match opt_payment {
                None => return None,
                Some(pay) => payment = pay,
            };
        }
        if !INCLUDE_IMPACT_OF_TRANSACTION {
            payment = payment / BigInt::from(WAD).pow(path.efective_pairs.len().try_into().unwrap());
        }
        Some(payment)
    }

    fn get_return_payment_from_efective_pair(&self, reserves: &Vec<BigInt>, efective_pair: &EfectivePair, payment: &BigInt) -> Option<BigInt> {
        let pair: &Pair = self.address_pairs.get(&efective_pair.address).unwrap();
        let internal_index = self.pair_string_addresses.iter().position(|r| r == &efective_pair.address).unwrap();
        let reserves_0: BigInt = reserves[internal_index * 2].clone();
        let reserves_1: BigInt = reserves[(internal_index * 2) + 1].clone();

        if DEBUG {
            println!("pair > {:?}", pair);
            println!("payment_in > {:?}", payment);
            println!("reserves_0 > {}", reserves_0);
            println!("reserves_1 > {}", reserves_1);
        }
        if efective_pair.token_in == pair.token_0 {
            let ret = self.get_out(payment, &reserves_0, &reserves_1);
            if DEBUG {
                println!("payment_out > {:?}", ret);
            }
            ret
        } else {
            let ret = self.get_out(payment, &reserves_1, &reserves_0);
            if DEBUG {
                println!("payment_out > {:?}", ret);
            }
            ret
        }
    }

    fn get_out(&self, payment_0: &BigInt, reserves_0: &BigInt, reserves_1: &BigInt) -> Option<BigInt> {
        if INCLUDE_IMPACT_OF_TRANSACTION {
            self.get_out1(payment_0, reserves_0, reserves_1)
        } else {
            self.get_out2(payment_0, reserves_0, reserves_1)
        }
    }

    fn get_out1(&self, payment_0: &BigInt, reserves_0: &BigInt, reserves_1: &BigInt) -> Option<BigInt> {
        let not_div_zero = reserves_0 > &BigInt::from(0u64);
        let minimum_reserves_ok = reserves_0 > &BigInt::from(MINIMUM_RESERVES) && reserves_1 > &BigInt::from(MINIMUM_RESERVES);
        if not_div_zero && minimum_reserves_ok {
            let wad = &BigInt::from(WAD);
            let com = &BigInt::from(COMMISSION_IN_WAD);
            let k = reserves_0 * reserves_1;
            let new_res0 = reserves_0 + payment_0;
            let new_res1 = &k * wad / new_res0;
            let calc = ((reserves_1 * wad) - &new_res1) / wad;
            Some(calc * (wad - com) / wad)
        } else {
            None
        }
    }

    fn get_out2(&self, payment_0: &BigInt, reserves_0: &BigInt, reserves_1: &BigInt) -> Option<BigInt> {
        let wad = &BigInt::from(WAD);
        let com = &BigInt::from(COMMISSION_IN_WAD);
        let not_div_zero = reserves_0 > &BigInt::from(0u64);
        let minimum_reserves_ok = reserves_0 > &BigInt::from(MINIMUM_RESERVES) && reserves_1 > &BigInt::from(MINIMUM_RESERVES);
        if not_div_zero && minimum_reserves_ok {
            let ratio = (reserves_1 * wad * wad) / reserves_0;
            let calc = (payment_0 * ratio) / wad;
            Some(calc * (wad - com) / wad)
        } else {
            None
        }
    }

    fn get_path_cost_in_wei(&self, reserves: &Vec<BigInt>, path: &Path, amount_in_wei: &BigInt) -> Option<BigInt> {
        let res = self.get_return_payment_from_path(reserves, path, amount_in_wei);
        match res {
            Some(num) => Some(BigInt::from(num)),
            None => None,
        }
    }
}
