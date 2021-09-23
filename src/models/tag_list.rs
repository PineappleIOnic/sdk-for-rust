use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TagList {
    pub sum: Option<i64>,
    pub tags: Option<Vec<Tag>>,
}

impl TagList {
    pub fn new(sum: Option<i64>, tags: Option<Vec<Tag>>) -> Self {
        TagList { sum , tags  }
    }
}