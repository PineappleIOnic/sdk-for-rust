use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub id: Option<String>,
    pub permissions: Option<Permissions>,
    pub name: Option<String>,
    pub dateCreated: Option<i64>,
    pub signature: Option<String>,
    pub mimeType: Option<String>,
    pub sizeOriginal: Option<i64>,
}

impl File {
    pub fn new(id: Option<String>, permissions: Option<Permissions>, name: Option<String>, dateCreated: Option<i64>, signature: Option<String>, mimeType: Option<String>, sizeOriginal: Option<i64>) -> Self {
        File { id , permissions , name , dateCreated , signature , mimeType , sizeOriginal  }
    }
}