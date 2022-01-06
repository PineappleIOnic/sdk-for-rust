#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use std::fmt::Display;
use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyOption<T> {
    Some(T),
    None {},
}


impl<T> From<EmptyOption<T>> for Option<T> {
    fn from(empty_option: EmptyOption<T>) -> Option<T> {
        match empty_option {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

impl<T> From<Option<T>> for EmptyOption<T> {
    fn from(option: Option<T>) -> EmptyOption<T> {
        match option {
            Some(option) => EmptyOption::Some(option),
            None {} => EmptyOption::None {},
        }
    }
}

impl<T> EmptyOption<T> {
    fn into_option(self) -> Option<T> {
        self.into()
    }
    fn as_option(&self) -> Option<&T> {
        match self {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locale {
        pub ip: String,
        pub countryCode: String,
        pub country: String,
        pub continentCode: String,
        pub continent: String,
        pub eu: bool,
        pub currency: String,
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.ip));
        formatBuffer.push_str(&format!("{}", self.countryCode));
        formatBuffer.push_str(&format!("{}", self.country));
        formatBuffer.push_str(&format!("{}", self.continentCode));
        formatBuffer.push_str(&format!("{}", self.continent));
        formatBuffer.push_str(&format!("{}", self.eu));
        formatBuffer.push_str(&format!("{}", self.currency));

        write!(f, "{}", formatBuffer)
    }
}

impl Locale {
    pub fn new(ip: String, countryCode: String, country: String, continentCode: String, continent: String, eu: bool, currency: String, ) -> Self {
        Self {
            ip: ip,
            countryCode: countryCode,
            country: country,
            continentCode: continentCode,
            continent: continent,
            eu: eu,
            currency: currency,
            }
    }
}