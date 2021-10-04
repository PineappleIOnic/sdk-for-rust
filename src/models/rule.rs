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
pub struct Rule {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "collection", deserialize = "$collection"))]
        pub collection: String,
        #[serde(rename(serialize = "xtype", deserialize = "type"))]
        pub xtype: String,
        pub key: String,
        pub label: String,
        pub default: String,
        pub array: bool,
        pub required: bool,
        pub list: Vec<String>,
}

impl Rule {
    pub fn new(id: String, collection: String, xtype: String, key: String, label: String, default: String, array: bool, required: bool, list: Vec<String>, ) -> Self {
        Self {
            id: id,
            collection: collection,
            xtype: xtype,
            key: key,
            label: label,
            default: default,
            array: array,
            required: required,
            list: list,
            }
    }
}