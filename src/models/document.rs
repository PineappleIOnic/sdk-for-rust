use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: Option<String>,
    pub collection: Option<String>,
    pub permissions: Option<Permissions>,
    pub data: HashMap<String, Value>,
}

impl Document {
    pub fn new(id: Option<String>, collection: Option<String>, permissions: Option<Permissions>, data: HashMap<String, Value>) -> Self {
        Document { id , collection , permissions , data }
    }
}