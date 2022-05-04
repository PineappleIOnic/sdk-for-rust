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
pub struct Function {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub execute: String,
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

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.id));
        formatBuffer.push_str(&format!("{}", self.execute));
        formatBuffer.push_str(&format!("{}", self.name));
        formatBuffer.push_str(&format!("{}", self.dateCreated));
        formatBuffer.push_str(&format!("{}", self.dateUpdated));
        formatBuffer.push_str(&format!("{}", self.status));
        formatBuffer.push_str(&format!("{}", self.runtime));
        formatBuffer.push_str(&format!("{}", self.tag));
        formatBuffer.push_str(&format!("{}", self.vars));
        for item in &self.events {
            formatBuffer.push_str(&format!("{}", item));
        }
        formatBuffer.push_str(&format!("{}", self.schedule));
        formatBuffer.push_str(&format!("{}", self.scheduleNext));
        formatBuffer.push_str(&format!("{}", self.schedulePrevious));
        formatBuffer.push_str(&format!("{}", self.timeout));

        write!(f, "{}", formatBuffer)
    }
}

impl Function {
    pub fn new(id: String, execute: String, name: String, dateCreated: i64, dateUpdated: i64, status: String, runtime: String, tag: String, vars: String, events: Vec<String>, schedule: String, scheduleNext: i64, schedulePrevious: i64, timeout: i64, ) -> Self {
        Self {
            id: id,
            execute: execute,
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