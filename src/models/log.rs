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
pub struct Log {
        pub event: EmptyOption<String>,
        pub ip: EmptyOption<String>,
        pub time: EmptyOption<i64>,
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
}

impl Log {
    pub fn new(event: EmptyOption<String>, ip: EmptyOption<String>, time: EmptyOption<i64>, osCode: EmptyOption<String>, osName: EmptyOption<String>, osVersion: EmptyOption<String>, clientType: EmptyOption<String>, clientCode: EmptyOption<String>, clientName: EmptyOption<String>, clientVersion: EmptyOption<String>, clientEngine: EmptyOption<String>, clientEngineVersion: EmptyOption<String>, deviceName: EmptyOption<String>, deviceBrand: EmptyOption<String>, deviceModel: EmptyOption<String>, countryCode: EmptyOption<String>, countryName: EmptyOption<String>, ) -> Self {
        Self {
            event: EmptyOption::from(event),
            ip: EmptyOption::from(ip),
            time: EmptyOption::from(time),
            osCode: EmptyOption::from(osCode),
            osName: EmptyOption::from(osName),
            osVersion: EmptyOption::from(osVersion),
            clientType: EmptyOption::from(clientType),
            clientCode: EmptyOption::from(clientCode),
            clientName: EmptyOption::from(clientName),
            clientVersion: EmptyOption::from(clientVersion),
            clientEngine: EmptyOption::from(clientEngine),
            clientEngineVersion: EmptyOption::from(clientEngineVersion),
            deviceName: EmptyOption::from(deviceName),
            deviceBrand: EmptyOption::from(deviceBrand),
            deviceModel: EmptyOption::from(deviceModel),
            countryCode: EmptyOption::from(countryCode),
            countryName: EmptyOption::from(countryName),
            }
    }
}