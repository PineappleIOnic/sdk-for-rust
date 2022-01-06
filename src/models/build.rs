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
pub struct Build {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub dateCreated: i64,
        pub status: String,
        pub stdout: String,
        pub stderr: String,
        pub buildTime: i64,
}

impl Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.id));
        formatBuffer.push_str(&format!("{}", self.dateCreated));
        formatBuffer.push_str(&format!("{}", self.status));
        formatBuffer.push_str(&format!("{}", self.stdout));
        formatBuffer.push_str(&format!("{}", self.stderr));
        formatBuffer.push_str(&format!("{}", self.buildTime));

        write!(f, "{}", formatBuffer)
    }
}

impl Build {
    pub fn new(id: String, dateCreated: i64, status: String, stdout: String, stderr: String, buildTime: i64, ) -> Self {
        Self {
            id: id,
            dateCreated: dateCreated,
            status: status,
            stdout: stdout,
            stderr: stderr,
            buildTime: buildTime,
            }
    }
}