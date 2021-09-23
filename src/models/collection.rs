use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub id: Option<String>,
    pub permissions: Option<Permissions>,
    pub name: Option<String>,
    pub dateCreated: Option<i64>,
    pub dateUpdated: Option<i64>,
    pub rules: Option<Vec<Rule>>,
}

impl Collection {
    pub fn new(id: Option<String>, permissions: Option<Permissions>, name: Option<String>, dateCreated: Option<i64>, dateUpdated: Option<i64>, rules: Option<Vec<Rule>>) -> Self {
        Collection { id , permissions , name , dateCreated , dateUpdated , rules  }
    }
}