use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Language {
    name: String,
    code: String,
    nativeName: String,
}

impl Language {
    pub fn new(name: String, code: String, nativeName: String) -> Self {
        Language { name , code , nativeName  }
    }
}