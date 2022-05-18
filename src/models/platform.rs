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
pub struct Platform {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub name: String,
        #[serde(rename(serialize = "xtype", deserialize = "type"))]
        pub xtype: String,
        pub key: String,
        pub store: String,
        pub hostname: String,
        pub httpUser: String,
        pub httpPass: String,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        formatBuffer.push_str(&format!("{:?}", self.name));
        formatBuffer.push_str(&format!("{:?}", self.xtype));
        formatBuffer.push_str(&format!("{:?}", self.key));
        formatBuffer.push_str(&format!("{:?}", self.store));
        formatBuffer.push_str(&format!("{:?}", self.hostname));
        formatBuffer.push_str(&format!("{:?}", self.httpUser));
        formatBuffer.push_str(&format!("{:?}", self.httpPass));

        write!(f, "{}", formatBuffer)
    }
}

impl Platform {
    pub fn new(id: String, name: String, xtype: String, key: String, store: String, hostname: String, httpUser: String, httpPass: String, ) -> Self {
        Self {
            id: id,
            name: name,
            xtype: xtype,
            key: key,
            store: store,
            hostname: hostname,
            httpUser: httpUser,
            httpPass: httpPass,
            }
    }
}