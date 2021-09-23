use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    pub code: Option<String>,
    pub countryCode: Option<String>,
    pub countryName: Option<String>,
}

impl Phone {
    pub fn new(code: Option<String>, countryCode: Option<String>, countryName: Option<String>) -> Self {
        Phone { code , countryCode , countryName  }
    }
}