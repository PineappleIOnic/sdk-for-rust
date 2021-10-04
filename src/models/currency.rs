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
pub struct Currency {
        pub symbol: EmptyOption<String>,
        pub name: EmptyOption<String>,
        pub symbolNative: EmptyOption<String>,
        pub decimalDigits: EmptyOption<i64>,
        pub rounding: EmptyOption<f64>,
        pub code: EmptyOption<String>,
        pub namePlural: EmptyOption<String>,
}

impl Currency {
    pub fn new(symbol: EmptyOption<String>, name: EmptyOption<String>, symbolNative: EmptyOption<String>, decimalDigits: EmptyOption<i64>, rounding: EmptyOption<f64>, code: EmptyOption<String>, namePlural: EmptyOption<String>, ) -> Self {
        Self {
            symbol: EmptyOption::from(symbol),
            name: EmptyOption::from(name),
            symbolNative: EmptyOption::from(symbolNative),
            decimalDigits: EmptyOption::from(decimalDigits),
            rounding: EmptyOption::from(rounding),
            code: EmptyOption::from(code),
            namePlural: EmptyOption::from(namePlural),
            }
    }
}