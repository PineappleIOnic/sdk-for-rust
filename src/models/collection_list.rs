use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CollectionList {
    pub sum: i64,
    pub collections: Vec<Collection>,
}

impl CollectionList {
    pub fn new(sum: i64, collections: Vec<Collection>) -> Self {
        CollectionList { sum , collections  }
    }
}