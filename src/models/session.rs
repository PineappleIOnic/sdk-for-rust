use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Session {
    id: String,
    userId: String,
    expire: i64,
    provider: String,
    providerUid: String,
    providerToken: String,
    ip: String,
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
    current: bool,
}

impl Session {
    pub fn new(id: String, userId: String, expire: i64, provider: String, providerUid: String, providerToken: String, ip: String, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String, current: bool) -> Self {
        Session { id , userId , expire , provider , providerUid , providerToken , ip , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName , current  }
    }
}