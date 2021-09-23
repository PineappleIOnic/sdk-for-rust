use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: Option<String>,
    pub userId: Option<String>,
    pub secret: Option<String>,
    pub expire: Option<i64>,
}

impl Token {
    pub fn new(id: Option<String>, userId: Option<String>, secret: Option<String>, expire: Option<i64>) -> Self {
        Token { id , userId , secret , expire  }
    }
}