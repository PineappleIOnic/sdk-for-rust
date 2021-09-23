use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Document {
    id: String,
    collection: String,
    permissions: Permissions,
    data: HashMap<String, Value>,
}

impl Document {
    pub fn new(id: String, collection: String, permissions: Permissions, data: HashMap<String, Value>) -> Self {
        Document { id , collection , permissions , data }
    }
}