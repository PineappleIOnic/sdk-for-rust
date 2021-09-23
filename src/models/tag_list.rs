use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TagList {
    pub sum: i64,
    pub tags: Vec<Tag>,
}

impl TagList {
    pub fn new(sum: i64, tags: Vec<Tag>) -> Self {
        TagList { sum , tags  }
    }
}