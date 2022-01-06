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
pub struct File {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "read", deserialize = "$read"))]
        pub read: Vec<String>,
        #[serde(rename(serialize = "write", deserialize = "$write"))]
        pub write: Vec<String>,
        pub name: String,
        pub dateCreated: i64,
        pub signature: String,
        pub mimeType: String,
        pub sizeOriginal: i64,
}

impl Display for File {
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
        formatBuffer.push_str(&format!("{}", self.dateCreated));
        formatBuffer.push_str(&format!("{}", self.signature));
        formatBuffer.push_str(&format!("{}", self.mimeType));
        formatBuffer.push_str(&format!("{}", self.sizeOriginal));

        write!(f, "{}", formatBuffer)
    }
}

impl File {
    pub fn new(id: String, read: Vec<String>, write: Vec<String>, name: String, dateCreated: i64, signature: String, mimeType: String, sizeOriginal: i64, ) -> Self {
        Self {
            id: id,
            read: read,
            write: write,
            name: name,
            dateCreated: dateCreated,
            signature: signature,
            mimeType: mimeType,
            sizeOriginal: sizeOriginal,
            }
    }
}