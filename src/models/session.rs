use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: Option<String>,
    pub userId: Option<String>,
    pub expire: Option<i64>,
    pub provider: Option<String>,
    pub providerUid: Option<String>,
    pub providerToken: Option<String>,
    pub ip: Option<String>,
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
    pub current: Option<bool>,
}

impl Session {
    pub fn new(id: Option<String>, userId: Option<String>, expire: Option<i64>, provider: Option<String>, providerUid: Option<String>, providerToken: Option<String>, ip: Option<String>, osCode: Option<String>, osName: Option<String>, osVersion: Option<String>, clientType: Option<String>, clientCode: Option<String>, clientName: Option<String>, clientVersion: Option<String>, clientEngine: Option<String>, clientEngineVersion: Option<String>, deviceName: Option<String>, deviceBrand: Option<String>, deviceModel: Option<String>, countryCode: Option<String>, countryName: Option<String>, current: Option<bool>) -> Self {
        Session { id , userId , expire , provider , providerUid , providerToken , ip , osCode , osName , osVersion , clientType , clientCode , clientName , clientVersion , clientEngine , clientEngineVersion , deviceName , deviceBrand , deviceModel , countryCode , countryName , current  }
    }
}