use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Team {
    id: String,
    name: String,
    dateCreated: i64,
    sum: i64,
}

impl Team {
    pub fn new(id: String, name: String, dateCreated: i64, sum: i64) -> Self {
        Team { id , name , dateCreated , sum  }
    }
}