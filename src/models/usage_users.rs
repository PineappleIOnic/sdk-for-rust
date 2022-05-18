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
pub struct UsageUsers {
        pub range: String,
        pub usersCount: Vec<MetricList>,
        pub usersCreate: Vec<MetricList>,
        pub usersRead: Vec<MetricList>,
        pub usersUpdate: Vec<MetricList>,
        pub usersDelete: Vec<MetricList>,
        pub sessionsCreate: Vec<MetricList>,
        pub sessionsProviderCreate: Vec<MetricList>,
        pub sessionsDelete: Vec<MetricList>,
}

impl Display for UsageUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.range));
        for item in &self.usersCount {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.usersCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.usersRead {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.usersUpdate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.usersDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.sessionsCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.sessionsProviderCreate {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.sessionsDelete {
            formatBuffer.push_str(&format!("{:?}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl UsageUsers {
    pub fn new(range: String, usersCount: Vec<MetricList>, usersCreate: Vec<MetricList>, usersRead: Vec<MetricList>, usersUpdate: Vec<MetricList>, usersDelete: Vec<MetricList>, sessionsCreate: Vec<MetricList>, sessionsProviderCreate: Vec<MetricList>, sessionsDelete: Vec<MetricList>, ) -> Self {
        Self {
            range: range,
            usersCount: usersCount,
            usersCreate: usersCreate,
            usersRead: usersRead,
            usersUpdate: usersUpdate,
            usersDelete: usersDelete,
            sessionsCreate: sessionsCreate,
            sessionsProviderCreate: sessionsProviderCreate,
            sessionsDelete: sessionsDelete,
            }
    }
}