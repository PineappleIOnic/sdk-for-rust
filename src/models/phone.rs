use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    pub code: String,
    pub countryCode: String,
    pub countryName: String,
}

impl Phone {
    pub fn new(code: String, countryCode: String, countryName: String) -> Self {
        Phone { code , countryCode , countryName  }
    }
}