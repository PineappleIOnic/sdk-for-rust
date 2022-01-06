#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use std::fmt::Display;
use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyOption<T> {
    Some(T),
    None {},
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
pub struct Collection {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "read", deserialize = "$read"))]
        pub read: Vec<String>,
        #[serde(rename(serialize = "write", deserialize = "$write"))]
        pub write: Vec<String>,
        pub name: String,
        pub permission: String,
        pub attributes: Vec<String>,
        pub indexes: Vec<Index>,
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.id));
        for item in &self.read {
            formatBuffer.push_str(&format!("{}", item));
        }
        for item in &self.write {
            formatBuffer.push_str(&format!("{}", item));
        }
        formatBuffer.push_str(&format!("{}", self.name));
        formatBuffer.push_str(&format!("{}", self.permission));
        for item in &self.attributes {
            formatBuffer.push_str(&format!("{}", item));
        }
        for item in &self.indexes {
            formatBuffer.push_str(&format!("{}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl Collection {
    pub fn new(id: String, read: Vec<String>, write: Vec<String>, name: String, permission: String, attributes: Vec<String>, indexes: Vec<Index>, ) -> Self {
        Self {
            id: id,
            read: read,
            write: write,
            name: name,
            permission: permission,
            attributes: attributes,
            indexes: indexes,
            }
    }
}