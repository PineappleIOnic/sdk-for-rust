use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub permissions: Permissions,
    pub name: String,
    pub dateCreated: i64,
    pub dateUpdated: i64,
    pub rules: Vec<Rule>,
}

impl Collection {
    pub fn new(id: String, permissions: Permissions, name: String, dateCreated: i64, dateUpdated: i64, rules: Vec<Rule>) -> Self {
        Collection { id , permissions , name , dateCreated , dateUpdated , rules  }
    }
}