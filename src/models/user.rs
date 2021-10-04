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
pub struct User {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: EmptyOption<String>,
        pub name: EmptyOption<String>,
        pub registration: EmptyOption<i64>,
        pub status: EmptyOption<i64>,
        pub passwordUpdate: EmptyOption<i64>,
        pub email: EmptyOption<String>,
        pub emailVerification: EmptyOption<bool>,
        pub prefs: EmptyOption<Preferences>,
}

impl User {
    pub fn new(id: EmptyOption<String>, name: EmptyOption<String>, registration: EmptyOption<i64>, status: EmptyOption<i64>, passwordUpdate: EmptyOption<i64>, email: EmptyOption<String>, emailVerification: EmptyOption<bool>, prefs: EmptyOption<Preferences>, ) -> Self {
        Self {
            id: EmptyOption::from(id),
            name: EmptyOption::from(name),
            registration: EmptyOption::from(registration),
            status: EmptyOption::from(status),
            passwordUpdate: EmptyOption::from(passwordUpdate),
            email: EmptyOption::from(email),
            emailVerification: EmptyOption::from(emailVerification),
            prefs: EmptyOption::from(prefs),
            }
    }
}