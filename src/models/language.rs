use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub name: Option<String>,
    pub code: Option<String>,
    pub nativeName: Option<String>,
}

impl Language {
    pub fn new(name: Option<String>, code: Option<String>, nativeName: Option<String>) -> Self {
        Language { name , code , nativeName  }
    }
}