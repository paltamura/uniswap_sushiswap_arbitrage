use crate::constants::*;
use serde::{de, Serialize};
use std::fs::{self, File};

pub fn read_json<T>(file_name: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: de::DeserializeOwned,
{
    let file_path = format!("{}{}", DATA_FOLDER, file_name);
    match fs::read_to_string(file_path.clone()) {
        Ok(json_str) => match serde_json::from_str(&json_str) {
            Ok(json) => Ok(json),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}

pub fn save_json<T>(serializable: &T, name: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: ?Sized + Serialize,
{
    let file_path = format!("{}{}", DATA_FOLDER, name);
    match File::create(file_path.clone()) {
        Ok(file) => match serde_json::to_writer(file, serializable) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}

pub fn cartesian_product(lists: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for &i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for &el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}
