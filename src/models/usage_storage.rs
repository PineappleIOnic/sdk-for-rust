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
pub struct UsageStorage {
        pub range: String,
        pub filesStorage: Vec<MetricList>,
        pub tagsStorage: Vec<MetricList>,
        pub filesCount: Vec<MetricList>,
        pub bucketsCount: Vec<MetricList>,
        pub bucketsCreate: Vec<MetricList>,
        pub bucketsRead: Vec<MetricList>,
        pub bucketsUpdate: Vec<MetricList>,
        pub bucketsDelete: Vec<MetricList>,
        pub filesCreate: Vec<MetricList>,
        pub filesRead: Vec<MetricList>,
        pub filesUpdate: Vec<MetricList>,
        pub filesDelete: Vec<MetricList>,
}

impl Display for UsageStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.range));
        for item in &self.filesStorage {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.tagsStorage {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.filesCount {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.bucketsCount {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.bucketsCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.bucketsRead {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.bucketsUpdate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.bucketsDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.filesCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.filesRead {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.filesUpdate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.filesDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl UsageStorage {
    pub fn new(range: String, filesStorage: Vec<MetricList>, tagsStorage: Vec<MetricList>, filesCount: Vec<MetricList>, bucketsCount: Vec<MetricList>, bucketsCreate: Vec<MetricList>, bucketsRead: Vec<MetricList>, bucketsUpdate: Vec<MetricList>, bucketsDelete: Vec<MetricList>, filesCreate: Vec<MetricList>, filesRead: Vec<MetricList>, filesUpdate: Vec<MetricList>, filesDelete: Vec<MetricList>, ) -> Self {
        Self {
            range: range,
            filesStorage: filesStorage,
            tagsStorage: tagsStorage,
            filesCount: filesCount,
            bucketsCount: bucketsCount,
            bucketsCreate: bucketsCreate,
            bucketsRead: bucketsRead,
            bucketsUpdate: bucketsUpdate,
            bucketsDelete: bucketsDelete,
            filesCreate: filesCreate,
            filesRead: filesRead,
            filesUpdate: filesUpdate,
            filesDelete: filesDelete,
            }
    }
}