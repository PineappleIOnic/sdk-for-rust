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
pub struct Index {
        pub key: String,
        #[serde(rename(serialize = "xtype", deserialize = "type"))]
        pub xtype: String,
        pub status: String,
        pub attributes: Vec<String>,
        pub orders: Vec<String>,
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.key));
        formatBuffer.push_str(&format!("{}", self.xtype));
        formatBuffer.push_str(&format!("{}", self.status));
        for item in &self.attributes {
            formatBuffer.push_str(&format!("{}", item));
        }
        for item in &self.orders {
            formatBuffer.push_str(&format!("{}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl Index {
    pub fn new(key: String, xtype: String, status: String, attributes: Vec<String>, orders: Vec<String>, ) -> Self {
        Self {
            key: key,
            xtype: xtype,
            status: status,
            attributes: attributes,
            orders: orders,
            }
    }
}