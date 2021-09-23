use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    pub name: Option<String>,
    pub code: Option<String>,
}

impl Continent {
    pub fn new(name: Option<String>, code: Option<String>) -> Self {
        Continent { name , code  }
    }
}