use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Log {
    event: String,
    ip: String,
    time: i64,
    osCode: String,
    osName: String,
    osVersion: String,
    clientType: String,
    clientCode: String,
    clientName: String,
    clientVersion: String,
    clientEngine: String,
    clientEngineVersion: String,
    deviceName: String,
    deviceBrand: String,
    deviceModel: String,
    countryCode: String,
    countryName: String,
}

impl Log {
    pub fn new(event: String, ip: String, time: i64, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String) -> Self {
        Log { event , ip , time , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName  }
    }
}