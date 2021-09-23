use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Permissions {
    pub read: Vec<String>,
    pub write: Vec<String>,
}

impl Permissions {
    pub fn new(read: &[&str], write: &[&str]) -> Self {
        Permissions { read: read.iter().map(|&s| s.to_string()).collect() , write: write.iter().map(|&s| s.to_string()).collect()  }
    }
}