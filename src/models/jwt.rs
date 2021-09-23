use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    jwt: String,
}

impl Jwt {
    pub fn new(jwt: String) -> Self {
        Jwt { jwt  }
    }
}