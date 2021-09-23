use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct LanguageList {
    sum: i64,
    languages: Vec<Language>,
}

impl LanguageList {
    pub fn new(sum: i64, languages: Vec<Language>) -> Self {
        LanguageList { sum , languages  }
    }
}