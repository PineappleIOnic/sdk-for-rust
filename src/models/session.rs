#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: EmptyOption<String>,
        pub userId: EmptyOption<String>,
        pub expire: EmptyOption<i64>,
        pub provider: EmptyOption<String>,
        pub providerUid: EmptyOption<String>,
        pub providerToken: EmptyOption<String>,
        pub ip: EmptyOption<String>,
        pub osCode: EmptyOption<String>,
        pub osName: EmptyOption<String>,
        pub osVersion: EmptyOption<String>,
        pub clientType: EmptyOption<String>,
        pub clientCode: EmptyOption<String>,
        pub clientName: EmptyOption<String>,
        pub clientVersion: EmptyOption<String>,
        pub clientEngine: EmptyOption<String>,
        pub clientEngineVersion: EmptyOption<String>,
        pub deviceName: EmptyOption<String>,
        pub deviceBrand: EmptyOption<String>,
        pub deviceModel: EmptyOption<String>,
        pub countryCode: EmptyOption<String>,
        pub countryName: EmptyOption<String>,
        pub current: EmptyOption<bool>,
}