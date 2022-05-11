#![allow(unused)]
use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use serde_json::value::Value;
use std::fmt::Display;
use super::*;

#[derive(Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyOption<T> {
    Some(T),
    None {},
}

impl<T> Display for EmptyOption<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmptyOption::Some(t) => write!(f, "{}", t),
            EmptyOption::None {} => write!(f, ""),
        }
    }
}

impl<'de, T> Deserialize<'de> for EmptyOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
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
pub struct Session {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub userId: String,
        pub expire: i64,
        pub provider: String,
        pub providerUid: String,
        pub providerAccessToken: String,
        pub providerAccessTokenExpiry: i64,
        pub providerRefreshToken: String,
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

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        formatBuffer.push_str(&format!("{:?}", self.userId));
        formatBuffer.push_str(&format!("{:?}", self.expire));
        formatBuffer.push_str(&format!("{:?}", self.provider));
        formatBuffer.push_str(&format!("{:?}", self.providerUid));
        formatBuffer.push_str(&format!("{:?}", self.providerAccessToken));
        formatBuffer.push_str(&format!("{:?}", self.providerAccessTokenExpiry));
        formatBuffer.push_str(&format!("{:?}", self.providerRefreshToken));
        formatBuffer.push_str(&format!("{:?}", self.ip));
        formatBuffer.push_str(&format!("{:?}", self.osCode));
        formatBuffer.push_str(&format!("{:?}", self.osName));
        formatBuffer.push_str(&format!("{:?}", self.osVersion));
        formatBuffer.push_str(&format!("{:?}", self.clientType));
        formatBuffer.push_str(&format!("{:?}", self.clientCode));
        formatBuffer.push_str(&format!("{:?}", self.clientName));
        formatBuffer.push_str(&format!("{:?}", self.clientVersion));
        formatBuffer.push_str(&format!("{:?}", self.clientEngine));
        formatBuffer.push_str(&format!("{:?}", self.clientEngineVersion));
        formatBuffer.push_str(&format!("{:?}", self.deviceName));
        formatBuffer.push_str(&format!("{:?}", self.deviceBrand));
        formatBuffer.push_str(&format!("{:?}", self.deviceModel));
        formatBuffer.push_str(&format!("{:?}", self.countryCode));
        formatBuffer.push_str(&format!("{:?}", self.countryName));
        formatBuffer.push_str(&format!("{:?}", self.current));

        write!(f, "{}", formatBuffer)
    }
}

impl Session {
    pub fn new(id: String, userId: String, expire: i64, provider: String, providerUid: String, providerAccessToken: String, providerAccessTokenExpiry: i64, providerRefreshToken: String, ip: String, osCode: String, osName: String, osVersion: String, clientType: String, clientCode: String, clientName: String, clientVersion: String, clientEngine: String, clientEngineVersion: String, deviceName: String, deviceBrand: String, deviceModel: String, countryCode: String, countryName: String, current: bool, ) -> Self {
        Self {
            id: id,
            userId: userId,
            expire: expire,
            provider: provider,
            providerUid: providerUid,
            providerAccessToken: providerAccessToken,
            providerAccessTokenExpiry: providerAccessTokenExpiry,
            providerRefreshToken: providerRefreshToken,
            ip: ip,
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
            current: current,
            }
    }
}