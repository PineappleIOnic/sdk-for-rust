use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageList {
    pub sum: Option<i64>,
    pub languages: Option<Vec<Language>>,
}

impl LanguageList {
    pub fn new(sum: Option<i64>, languages: Option<Vec<Language>>) -> Self {
        LanguageList { sum , languages  }
    }
}