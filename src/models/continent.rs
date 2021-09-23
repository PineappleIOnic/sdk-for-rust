use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    pub name: String,
    pub code: String,
}

impl Continent {
    pub fn new(name: String, code: String) -> Self {
        Continent { name , code  }
    }
}