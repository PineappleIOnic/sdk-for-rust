use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct DocumentList {
    sum: i64,
    documents: Vec<Document>,
}

impl DocumentList {
    pub fn new(sum: i64, documents: Vec<Document>) -> Self {
        DocumentList { sum , documents  }
    }
}