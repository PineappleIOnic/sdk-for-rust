use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Locale {
    ip: String,
    countryCode: String,
    country: String,
    continentCode: String,
    continent: String,
    eu: bool,
    currency: String,
}

impl Locale {
    pub fn new(ip: String, countryCode: String, country: String, continentCode: String, continent: String, eu: bool, currency: String) -> Self {
        Locale { ip , countryCode , country , continentCode , continent , eu , currency  }
    }
}