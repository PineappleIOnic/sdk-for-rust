use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Preferences {
    data: HashMap<String, Value>,
}

impl Preferences {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Preferences { data }
    }
}