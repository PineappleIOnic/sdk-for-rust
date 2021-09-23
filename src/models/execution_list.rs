use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ExecutionList {
    sum: i64,
    executions: Vec<Execution>,
}

impl ExecutionList {
    pub fn new(sum: i64, executions: Vec<Execution>) -> Self {
        ExecutionList { sum , executions  }
    }
}