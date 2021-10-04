#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
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
        pub ip: EmptyOption<String>,
        pub countryCode: EmptyOption<String>,
        pub country: EmptyOption<String>,
        pub continentCode: EmptyOption<String>,
        pub continent: EmptyOption<String>,
        pub eu: EmptyOption<bool>,
        pub currency: EmptyOption<String>,
}

impl Locale {
    pub fn new(ip: EmptyOption<String>, countryCode: EmptyOption<String>, country: EmptyOption<String>, continentCode: EmptyOption<String>, continent: EmptyOption<String>, eu: EmptyOption<bool>, currency: EmptyOption<String>, ) -> Self {
        Self {
            ip: EmptyOption::from(ip),
            countryCode: EmptyOption::from(countryCode),
            country: EmptyOption::from(country),
            continentCode: EmptyOption::from(continentCode),
            continent: EmptyOption::from(continent),
            eu: EmptyOption::from(eu),
            currency: EmptyOption::from(currency),
            }
    }
}