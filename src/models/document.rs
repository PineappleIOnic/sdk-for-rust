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
pub struct Document {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "collection", deserialize = "$collection"))]
        pub collection: String,
        #[serde(rename(serialize = "read", deserialize = "$read"))]
        pub read: Vec<String>,
        #[serde(rename(serialize = "write", deserialize = "$write"))]
        pub write: Vec<String>,
        #[serde(default)]
        pub data: HashMap<String, Value>,
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        formatBuffer.push_str(&format!("{:?}", self.collection));
        for item in &self.read {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.write {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for (key, value) in &self.data {
            formatBuffer.push_str(&format!("{}", value));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl Document {
    pub fn new(id: String, collection: String, read: Vec<String>, write: Vec<String>, ) -> Self {
        Self {
            id: id,
            collection: collection,
            read: read,
            write: write,
            data: HashMap::new(),
}
    }
}