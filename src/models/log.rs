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
        pub event: String,
        pub ip: String,
        pub time: i64,
        pub osCode: String,
        pub osName: String,
        pub osVersion: String,
        pub clientType: String,
        pub clientCode: String,
        pub clientName: String,
        pub clientVersion: String,
        pub clientEngine: String,
        pub clientEngineVersion: String,
        pub deviceName: String,
        pub deviceBrand: String,
        pub deviceModel: String,
        pub countryCode: String,
        pub countryName: String,
}

impl Log {
    pub fn new(event: String, ip: String, time: i64, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String, ) -> Self {
        Self {
            event: event,
            ip: ip,
            time: time,
            osCode: osCode,
            osName: osName,
            osVersion: osVersion,
            clientType: clientType,
            clientCode: clientCode,
            clientName: clientName,
            clientVersion: clientVersion,
            clientEngine: clientEngine,
            clientEngineVersion: clientEngineVersion,
            deviceName: deviceName,
            deviceBrand: deviceBrand,
            deviceModel: deviceModel,
            countryCode: countryCode,
            countryName: countryName,
            }
    }
}