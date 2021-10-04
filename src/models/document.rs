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
pub struct Document {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        #[serde(rename(serialize = "collection", deserialize = "$collection"))]
        pub collection: String,
        #[serde(rename(serialize = "permissions", deserialize = "$permissions"))]
        pub permissions: Permissions,
    pub data: HashMap<String, Value>,
}

impl Document {
    pub fn new(id: String, collection: String, permissions: Permissions, ) -> Self {
        Self {
            id: id,
            collection: collection,
            permissions: permissions,
            data: HashMap::new(),
}
    }
}