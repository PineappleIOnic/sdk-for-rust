use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinentList {
    pub sum: Option<i64>,
    pub continents: Option<Vec<Continent>>,
}

impl ContinentList {
    pub fn new(sum: Option<i64>, continents: Option<Vec<Continent>>) -> Self {
        ContinentList { sum , continents  }
    }
}