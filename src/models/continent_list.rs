use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ContinentList {
    pub sum: i64,
    pub continents: Vec<Continent>,
}

impl ContinentList {
    pub fn new(sum: i64, continents: Vec<Continent>) -> Self {
        ContinentList { sum , continents  }
    }
}