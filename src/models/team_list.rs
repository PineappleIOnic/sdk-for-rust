use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct TeamList {
    pub sum: i64,
    pub teams: Vec<Team>,
}

impl TeamList {
    pub fn new(sum: i64, teams: Vec<Team>) -> Self {
        TeamList { sum , teams  }
    }
}