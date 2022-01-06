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
pub struct AttributeString {
        pub key: String,
        #[serde(rename(serialize = "xtype", deserialize = "type"))]
        pub xtype: String,
        pub status: String,
        pub required: bool,
        pub array: bool,
        pub size: String,
        pub default: String,
}

impl Display for AttributeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.key));
        formatBuffer.push_str(&format!("{}", self.xtype));
        formatBuffer.push_str(&format!("{}", self.status));
        formatBuffer.push_str(&format!("{}", self.required));
        formatBuffer.push_str(&format!("{}", self.array));
        formatBuffer.push_str(&format!("{}", self.size));
        formatBuffer.push_str(&format!("{}", self.default));

        write!(f, "{}", formatBuffer)
    }
}

impl AttributeString {
    pub fn new(key: String, xtype: String, status: String, required: bool, array: bool, size: String, default: String, ) -> Self {
        Self {
            key: key,
            xtype: xtype,
            status: status,
            required: required,
            array: array,
            size: size,
            default: default,
            }
    }
}