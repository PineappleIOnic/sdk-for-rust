use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileList {
    pub sum: Option<i64>,
    pub files: Option<Vec<File>>,
}

impl FileList {
    pub fn new(sum: Option<i64>, files: Option<Vec<File>>) -> Self {
        FileList { sum , files  }
    }
}