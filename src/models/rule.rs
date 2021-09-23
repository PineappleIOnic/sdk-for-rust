use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub id: Option<String>,
    pub collection: Option<String>,
    pub xtype: Option<String>,
    pub key: Option<String>,
    pub label: Option<String>,
    pub default: Option<String>,
    pub array: Option<bool>,
    pub required: Option<bool>,
    pub list: Option<Vec<String>>,
}

impl Rule {
    pub fn new(id: Option<String>, collection: Option<String>, xtype: Option<String>, key: Option<String>, label: Option<String>, default: Option<String>, array: Option<bool>, required: Option<bool>, list: Option<&[&str]>) -> Self {
        Rule { id , collection , xtype , key , label , default , array , required , list: match list {
            Some(data) => Some(data.iter().map(|&s| s.to_string()).collect()),
            None => None,
        }   }
    }
}