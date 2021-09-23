use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub event: Option<String>,
    pub ip: Option<String>,
    pub time: Option<i64>,
    pub osCode: Option<String>,
    pub osName: Option<String>,
    pub osVersion: Option<String>,
    pub clientType: Option<String>,
    pub clientCode: Option<String>,
    pub clientName: Option<String>,
    pub clientVersion: Option<String>,
    pub clientEngine: Option<String>,
    pub clientEngineVersion: Option<String>,
    pub deviceName: Option<String>,
    pub deviceBrand: Option<String>,
    pub deviceModel: Option<String>,
    pub countryCode: Option<String>,
    pub countryName: Option<String>,
}

impl Log {
    pub fn new(event: Option<String>, ip: Option<String>, time: Option<i64>, osCode: Option<String>, osName: Option<String>, osVersion: Option<String>, clientType: Option<String>, clientCode: Option<String>, clientName: Option<String>, clientVersion: Option<String>, clientEngine: Option<String>, clientEngineVersion: Option<String>, deviceName: Option<String>, deviceBrand: Option<String>, deviceModel: Option<String>, countryCode: Option<String>, countryName: Option<String>) -> Self {
        Log { event , ip , time , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName  }
    }
}