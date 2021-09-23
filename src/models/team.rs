use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: Option<String>,
    pub name: Option<String>,
    pub dateCreated: Option<i64>,
    pub sum: Option<i64>,
}

impl Team {
    pub fn new(id: Option<String>, name: Option<String>, dateCreated: Option<i64>, sum: Option<i64>) -> Self {
        Team { id , name , dateCreated , sum  }
    }
}