use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionList {
    pub sum: Option<i64>,
    pub sessions: Option<Vec<Session>>,
}

impl SessionList {
    pub fn new(sum: Option<i64>, sessions: Option<Vec<Session>>) -> Self {
        SessionList { sum , sessions  }
    }
}