use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Locale {
    pub ip: Option<String>,
    pub countryCode: Option<String>,
    pub country: Option<String>,
    pub continentCode: Option<String>,
    pub continent: Option<String>,
    pub eu: Option<bool>,
    pub currency: Option<String>,
}

impl Locale {
    pub fn new(ip: Option<String>, countryCode: Option<String>, country: Option<String>, continentCode: Option<String>, continent: Option<String>, eu: Option<bool>, currency: Option<String>) -> Self {
        Locale { ip , countryCode , country , continentCode , continent , eu , currency  }
    }
}