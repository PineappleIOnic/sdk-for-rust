use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct FunctionList {
    sum: i64,
    functions: Vec<Function>,
}

impl FunctionList {
    pub fn new(sum: i64, functions: Vec<Function>) -> Self {
        FunctionList { sum , functions  }
    }
}