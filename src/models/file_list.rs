use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileList {
    pub sum: i64,
    pub files: Vec<File>,
}

impl FileList {
    pub fn new(sum: i64, files: Vec<File>) -> Self {
        FileList { sum , files  }
    }
}