use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionList {
    pub sum: Option<i64>,
    pub functions: Option<Vec<Function>>,
}

impl FunctionList {
    pub fn new(sum: Option<i64>, functions: Option<Vec<Function>>) -> Self {
        FunctionList { sum , functions  }
    }
}