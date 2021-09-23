use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct SessionList {
    sum: i64,
    sessions: Vec<Session>,
}

impl SessionList {
    pub fn new(sum: i64, sessions: Vec<Session>) -> Self {
        SessionList { sum , sessions  }
    }
}