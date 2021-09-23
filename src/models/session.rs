use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub userId: String,
    pub expire: i64,
    pub provider: String,
    pub providerUid: String,
    pub providerToken: String,
    pub ip: String,
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
    pub current: bool,
}

impl Session {
    pub fn new(id: String, userId: String, expire: i64, provider: String, providerUid: String, providerToken: String, ip: String, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String, current: bool) -> Self {
        Session { id , userId , expire , provider , providerUid , providerToken , ip , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName , current  }
    }
}