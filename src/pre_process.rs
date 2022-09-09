use crate::{constants, ethereum_interaction::read_pair_info, models, utils};
use constants::*;
use models::*;
use std::{collections::HashSet, fs};
use utils::*;

pub fn pre_processing() -> Result<(), Box<dyn std::error::Error>> {
    let uni_sushi_paths_file_path = format!("{}{}", WORKING_FILES_FOLDER, UNI_SUSHI_PATHS_FILE);

    let all_expanded_paths = match read_json(&uni_sushi_paths_file_path) {
        Ok(json_paths) => {
            let mut all_expanded_paths = expand_paths(json_paths);
            if FILTER_SYMMETRIC {
                all_expanded_paths = remove_symmetric_paths(all_expanded_paths);
            }
            all_expanded_paths
        },
        Err(err) => return Err(err),
    };

    let selected_paths_file_path = format!("{}{}", WORKING_FILES_FOLDER, SELECTED_PATHS_FILE);

    match save_json::<Vec<Path>>(&all_expanded_paths, &selected_paths_file_path) {
        Err(err) => return Err(err),
        _ => {},
    }

    let relevant_efective_pairs = get_all_efective_pairs(all_expanded_paths);

    let pair_addresses = get_unique_pair_addresses(relevant_efective_pairs);
    let selected_pair_addresses_file_path = format!("{}{}", WORKING_FILES_FOLDER, SELECTED_PAIR_ADDRESSES_FILE);
    match save_json::<Vec<String>>(&pair_addresses, &selected_pair_addresses_file_path) {
        Err(err) => return Err(err),
        _ => {},
    }

    if UPDATE_PAIRS {
        let mut pairs: Vec<Pair> = vec![];
        let mut internal_index = 0usize;
        let total_pair_addresses = pair_addresses.len();
        for pair_address in pair_addresses {
            match read_pair_info(&pair_address) {
                Ok((name, symbol, token_0, token_1)) => {
                    println!("[{}/{}] {}: {}", internal_index, total_pair_addresses, pair_address, name);
                    pairs.push(Pair {
                        internal_index,
                        name,
                        symbol,
                        token_0,
                        token_1,
                        address: pair_address,
                    });
                    internal_index += 1usize;
                },
                Err(err) => return Err(err),
            }
        }
        match save_json::<Vec<Pair>>(&pairs, ALL_PAIRS_FILE) {
            Err(err) => return Err(err),
            _ => {},
        }
    }

    if FIX_TOKENS {
        let tokens_file_path = format!("{}{}{}", DATA_FOLDER, WORKING_FILES_FOLDER, TOKENS_FILE);
        match fs::read_to_string(tokens_file_path.clone()) {
            Ok(contents) => {
                let fixed_contents = format!("[{}]", contents).replace("}", "},").replace(",]", "]");
                match fs::write(tokens_file_path.clone(), fixed_contents) {
                    Err(err) => return Err(Box::new(err)),
                    _ => {},
                }
            },
            Err(err) => return Err(Box::new(err)),
        }
    }

    Ok(())
}

fn get_unique_pair_addresses(efective_pairs: Vec<EfectivePair>) -> Vec<String> {
    let mut addresses = HashSet::new();
    for efective_pair in efective_pairs {
        addresses.insert(efective_pair.address);
    }
    addresses.into_iter().collect()
}

fn get_all_efective_pairs(paths: Vec<Path>) -> Vec<EfectivePair> {
    let mut all_efective_pairs = HashSet::new();
    for path in paths {
        for efective_pair in path.efective_pairs {
            all_efective_pairs.insert(efective_pair);
        }
    }
    all_efective_pairs.into_iter().collect()
}

fn remove_symmetric_paths(paths: Vec<Path>) -> Vec<Path> {
    let mut select_paths: Vec<Path> = vec![];
    for path in paths {
        if !is_symmetric_path(&path) {
            select_paths.push(path);
        }
    }
    select_paths
}

fn is_symmetric_path(path: &Path) -> bool {
    let len = path.efective_pairs.len().clone();
    let is_par = len % 2 == 0;
    let count: usize;
    if is_par {
        count = len / 2;
    } else {
        count = (len - 1) / 2;
    }
    for i in 0..count {
        let p0 = &path.efective_pairs[i].address;
        let p1 = &path.efective_pairs[len - 1].address;
        if p0 != p1 {
            return false;
        }
    }
    true
}

fn expand_paths(paths: Vec<Vec<JsonNode>>) -> Vec<Path> {
    let mut expanded_paths: Vec<Path> = vec![];
    for path in paths {
        expanded_paths.append(&mut expand_path(&path));
    }
    expanded_paths
}

fn expand_path(path: &Vec<JsonNode>) -> Vec<Path> {
    let comb_indexes: Vec<Vec<usize>> = comb_indexes(path);
    let path_size = path.len();
    let comb_size: usize = comb_indexes.len();
    let mut paths = vec![];
    for i in 0..comb_size {
        let indexes = &comb_indexes[i];
        let mut efective_pairs: Vec<EfectivePair> = vec![];
        for j in 0..path_size {
            let node = &path[j];
            let index = indexes[j];
            let token_in = if j == 0 { WETH_ADDRESS.to_string() } else { path[j - 1].token_out.clone() };
            let efective_pair = EfectivePair {
                token_in,
                token_out: node.token_out.clone(),
                address: node.pair_providers_addresses[index].clone(),
            };
            efective_pairs.push(efective_pair);
        }
        let path = Path { efective_pairs };
        paths.push(path);
    }
    paths
}

fn comb_indexes(path: &Vec<JsonNode>) -> Vec<Vec<usize>> {
    let mut index_domain: Vec<Vec<usize>> = vec![];
    for node in path {
        let mut vec = vec![];
        for i in 0..node.pair_providers_addresses.len() {
            vec.push(i);
        }
        index_domain.push(vec);
    }
    cartesian_product(&index_domain)
}
