#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
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
pub struct Function {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "permissions", deserialize = "$permissions"))]
        pub permissions: Permissions,
        pub name: String,
        pub dateCreated: i64,
        pub dateUpdated: i64,
        pub status: String,
        pub runtime: String,
        pub tag: String,
        pub vars: String,
        pub events: Vec<String>,
        pub schedule: String,
        pub scheduleNext: i64,
        pub schedulePrevious: i64,
        pub timeout: i64,
}

impl Function {
    pub fn new(id: String, permissions: Permissions, name: String, dateCreated: i64, dateUpdated: i64, status: String, runtime: String, tag: String, vars: String, events: Vec<String>, schedule: String, scheduleNext: i64, schedulePrevious: i64, timeout: i64, ) -> Self {
        Self {
            id: id,
            permissions: permissions,
            name: name,
            dateCreated: dateCreated,
            dateUpdated: dateUpdated,
            status: status,
            runtime: runtime,
            tag: tag,
            vars: vars,
            events: events,
            schedule: schedule,
            scheduleNext: scheduleNext,
            schedulePrevious: schedulePrevious,
            timeout: timeout,
            }
    }
}