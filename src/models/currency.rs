use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Currency {
    symbol: String,
    name: String,
    symbolNative: String,
    decimalDigits: i64,
    rounding: f64,
    code: String,
    namePlural: String,
}

impl Currency {
    pub fn new(symbol: String, name: String, symbolNative: String, decimalDigits: i64, rounding: f64, code: String, namePlural: String) -> Self {
        Currency { symbol , name , symbolNative , decimalDigits , rounding , code , namePlural  }
    }
}