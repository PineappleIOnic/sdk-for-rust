use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamList {
    pub sum: Option<i64>,
    pub teams: Option<Vec<Team>>,
}

impl TeamList {
    pub fn new(sum: Option<i64>, teams: Option<Vec<Team>>) -> Self {
        TeamList { sum , teams  }
    }
}