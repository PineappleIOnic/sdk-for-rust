use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub code: String,
}

impl Country {
    pub fn new(name: String, code: String) -> Self {
        Country { name , code  }
    }
}