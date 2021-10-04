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
        pub id: EmptyOption<String>,
        #[serde(rename(serialize = "permissions", deserialize = "$permissions"))]
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

impl Function {
    pub fn new(id: EmptyOption<String>, permissions: EmptyOption<Permissions>, name: EmptyOption<String>, dateCreated: EmptyOption<i64>, dateUpdated: EmptyOption<i64>, status: EmptyOption<String>, runtime: EmptyOption<String>, tag: EmptyOption<String>, vars: EmptyOption<String>, events: EmptyOption<Vec<String>>, schedule: EmptyOption<String>, scheduleNext: EmptyOption<i64>, schedulePrevious: EmptyOption<i64>, timeout: EmptyOption<i64>, ) -> Self {
        Self {
            id: EmptyOption::from(id),
            permissions: EmptyOption::from(permissions),
            name: EmptyOption::from(name),
            dateCreated: EmptyOption::from(dateCreated),
            dateUpdated: EmptyOption::from(dateUpdated),
            status: EmptyOption::from(status),
            runtime: EmptyOption::from(runtime),
            tag: EmptyOption::from(tag),
            vars: EmptyOption::from(vars),
            events: EmptyOption::from(events),
            schedule: EmptyOption::from(schedule),
            scheduleNext: EmptyOption::from(scheduleNext),
            schedulePrevious: EmptyOption::from(schedulePrevious),
            timeout: EmptyOption::from(timeout),
            }
    }
}