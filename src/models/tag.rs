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
pub struct Tag {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub functionId: String,
        pub dateCreated: i64,
        pub entrypoint: String,
        pub size: i64,
        pub status: String,
        pub buildId: String,
        pub buildStdout: String,
        pub buildStderr: String,
        pub automaticDeploy: bool,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.id));
        formatBuffer.push_str(&format!("{}", self.functionId));
        formatBuffer.push_str(&format!("{}", self.dateCreated));
        formatBuffer.push_str(&format!("{}", self.entrypoint));
        formatBuffer.push_str(&format!("{}", self.size));
        formatBuffer.push_str(&format!("{}", self.status));
        formatBuffer.push_str(&format!("{}", self.buildId));
        formatBuffer.push_str(&format!("{}", self.buildStdout));
        formatBuffer.push_str(&format!("{}", self.buildStderr));
        formatBuffer.push_str(&format!("{}", self.automaticDeploy));

        write!(f, "{}", formatBuffer)
    }
}

impl Tag {
    pub fn new(id: String, functionId: String, dateCreated: i64, entrypoint: String, size: i64, status: String, buildId: String, buildStdout: String, buildStderr: String, automaticDeploy: bool, ) -> Self {
        Self {
            id: id,
            functionId: functionId,
            dateCreated: dateCreated,
            entrypoint: entrypoint,
            size: size,
            status: status,
            buildId: buildId,
            buildStdout: buildStdout,
            buildStderr: buildStderr,
            automaticDeploy: automaticDeploy,
            }
    }
}