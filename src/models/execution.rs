use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Execution {
    pub id: String,
    pub permissions: Permissions,
    pub functionId: String,
    pub dateCreated: i64,
    pub trigger: String,
    pub status: String,
    pub exitCode: i64,
    pub stdout: String,
    pub stderr: String,
    pub time: f64,
}

impl Execution {
    pub fn new(id: String, permissions: Permissions, functionId: String, dateCreated: i64, trigger: String, status: String, exitCode: i64, stdout: String, stderr: String, time: f64) -> Self {
        Execution { id , permissions , functionId , dateCreated , trigger , status , exitCode , stdout , stderr , time  }
    }
}