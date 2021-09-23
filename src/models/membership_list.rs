use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MembershipList {
    sum: i64,
    memberships: Vec<Membership>,
}

impl MembershipList {
    pub fn new(sum: i64, memberships: Vec<Membership>) -> Self {
        MembershipList { sum , memberships  }
    }
}