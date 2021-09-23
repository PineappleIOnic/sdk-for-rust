use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct PhoneList {
    pub sum: i64,
    pub phones: Vec<Phone>,
}

impl PhoneList {
    pub fn new(sum: i64, phones: Vec<Phone>) -> Self {
        PhoneList { sum , phones  }
    }
}