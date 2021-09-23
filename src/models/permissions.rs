use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub read: Option<Vec<String>>,
    pub write: Option<Vec<String>>,
}

impl Permissions {
    pub fn new(read: Option<&[&str]>, write: Option<&[&str]>) -> Self {
        Permissions { read: match read {
            Some(data) => Some(data.iter().map(|&s| s.to_string()).collect()),
            None => None,
        }  , write: match write {
            Some(data) => Some(data.iter().map(|&s| s.to_string()).collect()),
            None => None,
        }   }
    }
}