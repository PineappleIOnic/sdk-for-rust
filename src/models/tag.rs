use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Option<String>,
    pub functionId: Option<String>,
    pub dateCreated: Option<i64>,
    pub command: Option<String>,
    pub size: Option<String>,
}

impl Tag {
    pub fn new(id: Option<String>, functionId: Option<String>, dateCreated: Option<i64>, command: Option<String>, size: Option<String>) -> Self {
        Tag { id , functionId , dateCreated , command , size  }
    }
}