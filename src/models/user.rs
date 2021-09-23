use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    registration: i64,
    status: i64,
    passwordUpdate: i64,
    email: String,
    emailVerification: bool,
    prefs: Preferences,
}

impl User {
    pub fn new(id: String, name: String, registration: i64, status: i64, passwordUpdate: i64, email: String, emailVerification: bool, prefs: Preferences) -> Self {
        User { id , name , registration , status , passwordUpdate , email , emailVerification , prefs  }
    }
}