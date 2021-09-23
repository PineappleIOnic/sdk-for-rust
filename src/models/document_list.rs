use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentList {
    pub sum: i64,
    pub documents: Vec<Document>,
}

impl DocumentList {
    pub fn new(sum: i64, documents: Vec<Document>) -> Self {
        DocumentList { sum , documents  }
    }
}