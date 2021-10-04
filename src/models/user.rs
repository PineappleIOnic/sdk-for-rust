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
        pub id: String,
        pub name: String,
        pub registration: i64,
        pub status: i64,
        pub passwordUpdate: i64,
        pub email: String,
        pub emailVerification: bool,
        pub prefs: Preferences,
}

impl User {
    pub fn new(id: String, name: String, registration: i64, status: i64, passwordUpdate: i64, email: String, emailVerification: bool, prefs: Preferences, ) -> Self {
        Self {
            id: id,
            name: name,
            registration: registration,
            status: status,
            passwordUpdate: passwordUpdate,
            email: email,
            emailVerification: emailVerification,
            prefs: prefs,
            }
    }
}