use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Token {
    id: String,
    userId: String,
    secret: String,
    expire: i64,
}

impl Token {
    pub fn new(id: String, userId: String, secret: String, expire: i64) -> Self {
        Token { id , userId , secret , expire  }
    }
}