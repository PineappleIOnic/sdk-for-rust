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
pub struct UsageDatabase {
        pub range: String,
        pub documentsCount: Vec<MetricList>,
        pub collectionsCount: Vec<MetricList>,
        pub documentsCreate: Vec<MetricList>,
        pub documentsRead: Vec<MetricList>,
        pub documentsUpdate: Vec<MetricList>,
        pub documentsDelete: Vec<MetricList>,
        pub collectionsCreate: Vec<MetricList>,
        pub collectionsRead: Vec<MetricList>,
        pub collectionsUpdate: Vec<MetricList>,
        pub collectionsDelete: Vec<MetricList>,
}

impl Display for UsageDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.range));
        for item in &self.documentsCount {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collectionsCount {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.documentsCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.documentsRead {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.documentsUpdate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.documentsDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collectionsCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collectionsRead {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collectionsUpdate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.collectionsDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl UsageDatabase {
    pub fn new(range: String, documentsCount: Vec<MetricList>, collectionsCount: Vec<MetricList>, documentsCreate: Vec<MetricList>, documentsRead: Vec<MetricList>, documentsUpdate: Vec<MetricList>, documentsDelete: Vec<MetricList>, collectionsCreate: Vec<MetricList>, collectionsRead: Vec<MetricList>, collectionsUpdate: Vec<MetricList>, collectionsDelete: Vec<MetricList>, ) -> Self {
        Self {
            range: range,
            documentsCount: documentsCount,
            collectionsCount: collectionsCount,
            documentsCreate: documentsCreate,
            documentsRead: documentsRead,
            documentsUpdate: documentsUpdate,
            documentsDelete: documentsDelete,
            collectionsCreate: collectionsCreate,
            collectionsRead: collectionsRead,
            collectionsUpdate: collectionsUpdate,
            collectionsDelete: collectionsDelete,
            }
    }
}