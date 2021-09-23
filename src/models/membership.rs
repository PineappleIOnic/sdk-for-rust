use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    pub userId: String,
    pub teamId: String,
    pub name: String,
    pub email: String,
    pub invited: i64,
    pub joined: i64,
    pub confirm: bool,
    pub roles: Vec<String>,
}

impl Membership {
    pub fn new(id: String, userId: String, teamId: String, name: String, email: String, invited: i64, joined: i64, confirm: bool, roles: &[&str]) -> Self {
        Membership { id , userId , teamId , name , email , invited , joined , confirm , roles: roles.iter().map(|&s| s.to_string()).collect()  }
    }
}