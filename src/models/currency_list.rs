use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyList {
    pub sum: Option<i64>,
    pub currencies: Option<Vec<Currency>>,
}

impl CurrencyList {
    pub fn new(sum: Option<i64>, currencies: Option<Vec<Currency>>) -> Self {
        CurrencyList { sum , currencies  }
    }
}