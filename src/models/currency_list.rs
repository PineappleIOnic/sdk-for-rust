use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyList {
    pub sum: i64,
    pub currencies: Vec<Currency>,
}

impl CurrencyList {
    pub fn new(sum: i64, currencies: Vec<Currency>) -> Self {
        CurrencyList { sum , currencies  }
    }
}