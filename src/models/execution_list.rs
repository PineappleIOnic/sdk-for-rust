use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionList {
    pub sum: Option<i64>,
    pub executions: Option<Vec<Execution>>,
}

impl ExecutionList {
    pub fn new(sum: Option<i64>, executions: Option<Vec<Execution>>) -> Self {
        ExecutionList { sum , executions  }
    }
}