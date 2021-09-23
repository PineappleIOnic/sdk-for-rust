use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub userId: String,
    pub secret: String,
    pub expire: i64,
}

impl Token {
    pub fn new(id: String, userId: String, secret: String, expire: i64) -> Self {
        Token { id , userId , secret , expire  }
    }
}