use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Membership {
    pub id: Option<String>,
    pub userId: Option<String>,
    pub teamId: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub invited: Option<i64>,
    pub joined: Option<i64>,
    pub confirm: Option<bool>,
    pub roles: Option<Vec<String>>,
}

impl Membership {
    pub fn new(id: Option<String>, userId: Option<String>, teamId: Option<String>, name: Option<String>, email: Option<String>, invited: Option<i64>, joined: Option<i64>, confirm: Option<bool>, roles: Option<&[&str]>) -> Self {
        Membership { id , userId , teamId , name , email , invited , joined , confirm , roles: match roles {
            Some(data) => Some(data.iter().map(|&s| s.to_string()).collect()),
            None => None,
        }   }
    }
}