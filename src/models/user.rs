use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub registration: Option<i64>,
    pub status: Option<i64>,
    pub passwordUpdate: Option<i64>,
    pub email: Option<String>,
    pub emailVerification: Option<bool>,
    pub prefs: Option<Preferences>,
}

impl User {
    pub fn new(id: Option<String>, name: Option<String>, registration: Option<i64>, status: Option<i64>, passwordUpdate: Option<i64>, email: Option<String>, emailVerification: Option<bool>, prefs: Option<Preferences>) -> Self {
        User { id , name , registration , status , passwordUpdate , email , emailVerification , prefs  }
    }
}