use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogList {
    pub logs: Option<Vec<Log>>,
}

impl LogList {
    pub fn new(logs: Option<Vec<Log>>) -> Self {
        LogList { logs  }
    }
}