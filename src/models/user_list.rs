use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct UserList {
    pub sum: i64,
    pub users: Vec<User>,
}

impl UserList {
    pub fn new(sum: i64, users: Vec<User>) -> Self {
        UserList { sum , users  }
    }
}