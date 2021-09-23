use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentList {
    pub sum: Option<i64>,
    pub documents: Option<Vec<Document>>,
}

impl DocumentList {
    pub fn new(sum: Option<i64>, documents: Option<Vec<Document>>) -> Self {
        DocumentList { sum , documents  }
    }
}