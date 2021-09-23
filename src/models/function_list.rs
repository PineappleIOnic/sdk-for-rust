use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionList {
    pub sum: i64,
    pub functions: Vec<Function>,
}

impl FunctionList {
    pub fn new(sum: i64, functions: Vec<Function>) -> Self {
        FunctionList { sum , functions  }
    }
}