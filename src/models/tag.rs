use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    id: String,
    functionId: String,
    dateCreated: i64,
    command: String,
    size: String,
}

impl Tag {
    pub fn new(id: String, functionId: String, dateCreated: i64, command: String, size: String) -> Self {
        Tag { id , functionId , dateCreated , command , size  }
    }
}