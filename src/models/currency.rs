use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub symbolNative: Option<String>,
    pub decimalDigits: Option<i64>,
    pub rounding: Option<f64>,
    pub code: Option<String>,
    pub namePlural: Option<String>,
}

impl Currency {
    pub fn new(symbol: Option<String>, name: Option<String>, symbolNative: Option<String>, decimalDigits: Option<i64>, rounding: Option<f64>, code: Option<String>, namePlural: Option<String>) -> Self {
        Currency { symbol , name , symbolNative , decimalDigits , rounding , code , namePlural  }
    }
}