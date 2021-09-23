use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct LogList {
    pub logs: Vec<Log>,
}

impl LogList {
    pub fn new(logs: Vec<Log>) -> Self {
        LogList { logs  }
    }
}