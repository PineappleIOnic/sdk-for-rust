use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct File {
    id: String,
    permissions: Permissions,
    name: String,
    dateCreated: i64,
    signature: String,
    mimeType: String,
    sizeOriginal: i64,
}

impl File {
    pub fn new(id: String, permissions: Permissions, name: String, dateCreated: i64, signature: String, mimeType: String, sizeOriginal: i64) -> Self {
        File { id , permissions , name , dateCreated , signature , mimeType , sizeOriginal  }
    }
}