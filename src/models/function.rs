use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub id: Option<String>,
    pub permissions: Option<Permissions>,
    pub name: Option<String>,
    pub dateCreated: Option<i64>,
    pub dateUpdated: Option<i64>,
    pub status: Option<String>,
    pub runtime: Option<String>,
    pub tag: Option<String>,
    pub vars: Option<String>,
    pub events: Option<Vec<String>>,
    pub schedule: Option<String>,
    pub scheduleNext: Option<i64>,
    pub schedulePrevious: Option<i64>,
    pub timeout: Option<i64>,
}

impl Function {
    pub fn new(id: Option<String>, permissions: Option<Permissions>, name: Option<String>, dateCreated: Option<i64>, dateUpdated: Option<i64>, status: Option<String>, runtime: Option<String>, tag: Option<String>, vars: Option<String>, events: Option<&[&str]>, schedule: Option<String>, scheduleNext: Option<i64>, schedulePrevious: Option<i64>, timeout: Option<i64>) -> Self {
        Function { id , permissions , name , dateCreated , dateUpdated , status , runtime , tag , vars , events: match events {
            Some(data) => Some(data.iter().map(|&s| s.to_string()).collect()),
            None => None,
        }  , schedule , scheduleNext , schedulePrevious , timeout  }
    }
}