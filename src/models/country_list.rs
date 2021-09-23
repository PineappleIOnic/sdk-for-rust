use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CountryList {
    sum: i64,
    countries: Vec<Country>,
}

impl CountryList {
    pub fn new(sum: i64, countries: Vec<Country>) -> Self {
        CountryList { sum , countries  }
    }
}