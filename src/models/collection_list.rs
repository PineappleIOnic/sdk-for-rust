use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionList {
    pub sum: Option<i64>,
    pub collections: Option<Vec<Collection>>,
}

impl CollectionList {
    pub fn new(sum: Option<i64>, collections: Option<Vec<Collection>>) -> Self {
        CollectionList { sum , collections  }
    }
}