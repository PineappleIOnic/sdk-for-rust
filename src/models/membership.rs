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
pub struct Membership {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub userId: String,
        pub teamId: String,
        pub name: String,
        pub email: String,
        pub invited: i64,
        pub joined: i64,
        pub confirm: bool,
        pub roles: Vec<String>,
}

impl Membership {
    pub fn new(id: String, userId: String, teamId: String, name: String, email: String, invited: i64, joined: i64, confirm: bool, roles: Vec<String>, ) -> Self {
        Self {
            id: id,
            userId: userId,
            teamId: teamId,
            name: name,
            email: email,
            invited: invited,
            joined: joined,
            confirm: confirm,
            roles: roles,
            }
    }
}