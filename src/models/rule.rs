use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub id: String,
    pub collection: String,
    pub xtype: String,
    pub key: String,
    pub label: String,
    pub default: String,
    pub array: bool,
    pub required: bool,
    pub list: Vec<String>,
}

impl Rule {
    pub fn new(id: String, collection: String, xtype: String, key: String, label: String, default: String, array: bool, required: bool, list: &[&str]) -> Self {
        Rule { id , collection , xtype , key , label , default , array , required , list: list.iter().map(|&s| s.to_string()).collect()  }
    }
}