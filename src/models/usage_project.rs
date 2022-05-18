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
pub struct UsageProject {
        pub range: String,
        pub requests: Vec<MetricList>,
        pub network: Vec<MetricList>,
        pub functions: Vec<MetricList>,
        pub documents: Vec<MetricList>,
        pub collections: Vec<MetricList>,
        pub users: Vec<MetricList>,
        pub storage: Vec<MetricList>,
}

impl Display for UsageProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.range));
        for item in &self.requests {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.network {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.functions {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.documents {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collections {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.users {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.storage {
            formatBuffer.push_str(&format!("{:?}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl UsageProject {
    pub fn new(range: String, requests: Vec<MetricList>, network: Vec<MetricList>, functions: Vec<MetricList>, documents: Vec<MetricList>, collections: Vec<MetricList>, users: Vec<MetricList>, storage: Vec<MetricList>, ) -> Self {
        Self {
            range: range,
            requests: requests,
            network: network,
            functions: functions,
            documents: documents,
            collections: collections,
            users: users,
            storage: storage,
            }
    }
}