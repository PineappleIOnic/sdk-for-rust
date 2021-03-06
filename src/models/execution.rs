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
pub struct Execution {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "read", deserialize = "$read"))]
        pub read: Vec<String>,
        pub functionId: String,
        pub dateCreated: i64,
        pub trigger: String,
        pub status: String,
        pub statusCode: i64,
        pub stdout: String,
        pub stderr: String,
        pub time: f64,
}

impl Display for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        for item in &self.read {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        formatBuffer.push_str(&format!("{:?}", self.functionId));
        formatBuffer.push_str(&format!("{:?}", self.dateCreated));
        formatBuffer.push_str(&format!("{:?}", self.trigger));
        formatBuffer.push_str(&format!("{:?}", self.status));
        formatBuffer.push_str(&format!("{:?}", self.statusCode));
        formatBuffer.push_str(&format!("{:?}", self.stdout));
        formatBuffer.push_str(&format!("{:?}", self.stderr));
        formatBuffer.push_str(&format!("{:?}", self.time));

        write!(f, "{}", formatBuffer)
    }
}

impl Execution {
    pub fn new(id: String, read: Vec<String>, functionId: String, dateCreated: i64, trigger: String, status: String, statusCode: i64, stdout: String, stderr: String, time: f64, ) -> Self {
        Self {
            id: id,
            read: read,
            functionId: functionId,
            dateCreated: dateCreated,
            trigger: trigger,
            status: status,
            statusCode: statusCode,
            stdout: stdout,
            stderr: stderr,
            time: time,
            }
    }
}