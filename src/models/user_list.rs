use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    pub sum: Option<i64>,
    pub users: Option<Vec<User>>,
}

impl UserList {
    pub fn new(sum: Option<i64>, users: Option<Vec<User>>) -> Self {
        UserList { sum , users  }
    }
}