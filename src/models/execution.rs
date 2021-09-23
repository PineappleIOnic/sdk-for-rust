use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Execution {
    pub id: Option<String>,
    pub permissions: Option<Permissions>,
    pub functionId: Option<String>,
    pub dateCreated: Option<i64>,
    pub trigger: Option<String>,
    pub status: Option<String>,
    pub exitCode: Option<i64>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub time: Option<f64>,
}

impl Execution {
    pub fn new(id: Option<String>, permissions: Option<Permissions>, functionId: Option<String>, dateCreated: Option<i64>, trigger: Option<String>, status: Option<String>, exitCode: Option<i64>, stdout: Option<String>, stderr: Option<String>, time: Option<f64>) -> Self {
        Execution { id , permissions , functionId , dateCreated , trigger , status , exitCode , stdout , stderr , time  }
    }
}