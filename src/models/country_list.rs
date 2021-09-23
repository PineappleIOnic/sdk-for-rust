use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryList {
    pub sum: Option<i64>,
    pub countries: Option<Vec<Country>>,
}

impl CountryList {
    pub fn new(sum: Option<i64>, countries: Option<Vec<Country>>) -> Self {
        CountryList { sum , countries  }
    }
}