use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::value::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipList {
    pub sum: Option<i64>,
    pub memberships: Option<Vec<Membership>>,
}

impl MembershipList {
    pub fn new(sum: Option<i64>, memberships: Option<Vec<Membership>>) -> Self {
        MembershipList { sum , memberships  }
    }
}