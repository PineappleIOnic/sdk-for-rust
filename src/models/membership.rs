use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Membership {
    id: String,
    userId: String,
    teamId: String,
    name: String,
    email: String,
    invited: i64,
    joined: i64,
    confirm: bool,
    roles: Vec<String>,
}

impl Membership {
    pub fn new(id: String, userId: String, teamId: String, name: String, email: String, invited: i64, joined: i64, confirm: bool, roles: &[&str]) -> Self {
        Membership { id , userId , teamId , name , email , invited , joined , confirm , roles: roles.iter().map(|&s| s.to_string()).collect()  }
    }
}