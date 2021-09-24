#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub id: EmptyOption<String>,
    pub permissions: EmptyOption<Permissions>,
    pub name: EmptyOption<String>,
    pub dateCreated: EmptyOption<i64>,
    pub dateUpdated: EmptyOption<i64>,
    pub status: EmptyOption<String>,
    pub runtime: EmptyOption<String>,
    pub tag: EmptyOption<String>,
    pub vars: EmptyOption<String>,
    pub events: EmptyOption<Vec<String>>,
    pub schedule: EmptyOption<String>,
    pub scheduleNext: EmptyOption<i64>,
    pub schedulePrevious: EmptyOption<i64>,
    pub timeout: EmptyOption<i64>,
}