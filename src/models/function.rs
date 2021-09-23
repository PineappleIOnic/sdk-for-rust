use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Function {
    id: String,
    permissions: Permissions,
    name: String,
    dateCreated: i64,
    dateUpdated: i64,
    status: String,
    runtime: String,
    tag: String,
    vars: String,
    events: Vec<String>,
    schedule: String,
    scheduleNext: i64,
    schedulePrevious: i64,
    timeout: i64,
}

impl Function {
    pub fn new(id: String, permissions: Permissions, name: String, dateCreated: i64, dateUpdated: i64, status: String, runtime: String, tag: String, vars: String, events: &[&str], schedule: String, scheduleNext: i64, schedulePrevious: i64, timeout: i64) -> Self {
        Function { id , permissions , name , dateCreated , dateUpdated , status , runtime , tag , vars , events: events.iter().map(|&s| s.to_string()).collect() , schedule , scheduleNext , schedulePrevious , timeout  }
    }
}