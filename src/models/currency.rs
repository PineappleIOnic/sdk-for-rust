use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub symbol: String,
    pub name: String,
    pub symbolNative: String,
    pub decimalDigits: i64,
    pub rounding: f64,
    pub code: String,
    pub namePlural: String,
}

impl Currency {
    pub fn new(symbol: String, name: String, symbolNative: String, decimalDigits: i64, rounding: f64, code: String, namePlural: String) -> Self {
        Currency { symbol , name , symbolNative , decimalDigits , rounding , code , namePlural  }
    }
}