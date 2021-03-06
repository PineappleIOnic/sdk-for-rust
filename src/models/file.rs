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
pub struct File {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub bucketId: String,
        #[serde(rename(serialize = "read", deserialize = "$read"))]
        pub read: Vec<String>,
        #[serde(rename(serialize = "write", deserialize = "$write"))]
        pub write: Vec<String>,
        pub name: String,
        pub dateCreated: i64,
        pub signature: String,
        pub mimeType: String,
        pub sizeOriginal: i64,
        pub chunksTotal: i64,
        pub chunksUploaded: i64,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        formatBuffer.push_str(&format!("{:?}", self.bucketId));
        for item in &self.read {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.write {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        formatBuffer.push_str(&format!("{:?}", self.name));
        formatBuffer.push_str(&format!("{:?}", self.dateCreated));
        formatBuffer.push_str(&format!("{:?}", self.signature));
        formatBuffer.push_str(&format!("{:?}", self.mimeType));
        formatBuffer.push_str(&format!("{:?}", self.sizeOriginal));
        formatBuffer.push_str(&format!("{:?}", self.chunksTotal));
        formatBuffer.push_str(&format!("{:?}", self.chunksUploaded));

        write!(f, "{}", formatBuffer)
    }
}

impl File {
    pub fn new(id: String, bucketId: String, read: Vec<String>, write: Vec<String>, name: String, dateCreated: i64, signature: String, mimeType: String, sizeOriginal: i64, chunksTotal: i64, chunksUploaded: i64, ) -> Self {
        Self {
            id: id,
            bucketId: bucketId,
            read: read,
            write: write,
            name: name,
            dateCreated: dateCreated,
            signature: signature,
            mimeType: mimeType,
            sizeOriginal: sizeOriginal,
            chunksTotal: chunksTotal,
            chunksUploaded: chunksUploaded,
            }
    }
}