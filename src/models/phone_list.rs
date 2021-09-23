use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneList {
    pub sum: Option<i64>,
    pub phones: Option<Vec<Phone>>,
}

impl PhoneList {
    pub fn new(sum: Option<i64>, phones: Option<Vec<Phone>>) -> Self {
        PhoneList { sum , phones  }
    }
}