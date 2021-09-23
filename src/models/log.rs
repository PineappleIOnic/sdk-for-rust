use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new(event: String, ip: String, time: i64, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String) -> Self {
        Log { event , ip , time , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName  }
    }
}