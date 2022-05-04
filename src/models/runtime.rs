#![allow(unused)]
use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use serde_json::value::Value;
use std::fmt::Display;
use super::*;

#[derive(Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyOption<T> {
    Some(T),
    None {},
}

impl<T> Display for EmptyOption<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmptyOption::Some(t) => write!(f, "{}", t),
            EmptyOption::None {} => write!(f, ""),
        }
    }
}

impl<'de, T> Deserialize<'de> for EmptyOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T> From<EmptyOption<T>> for Option<T> {
    fn from(empty_option: EmptyOption<T>) -> Option<T> {
        match empty_option {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

impl<T> From<Option<T>> for EmptyOption<T> {
    fn from(option: Option<T>) -> EmptyOption<T> {
        match option {
            Some(option) => EmptyOption::Some(option),
            None {} => EmptyOption::None {},
        }
    }
}

impl<T> EmptyOption<T> {
    fn into_option(self) -> Option<T> {
        self.into()
    }
    fn as_option(&self) -> Option<&T> {
        match self {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runtime {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub name: String,
        pub version: String,
        pub base: String,
        pub image: String,
        pub logo: String,
        pub supports: Vec<String>,
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{}", self.id));
        formatBuffer.push_str(&format!("{}", self.name));
        formatBuffer.push_str(&format!("{}", self.version));
        formatBuffer.push_str(&format!("{}", self.base));
        formatBuffer.push_str(&format!("{}", self.image));
        formatBuffer.push_str(&format!("{}", self.logo));
        for item in &self.supports {
            formatBuffer.push_str(&format!("{}", item));
        }

        write!(f, "{}", formatBuffer)
    }
}

impl Runtime {
    pub fn new(id: String, name: String, version: String, base: String, image: String, logo: String, supports: Vec<String>, ) -> Self {
        Self {
            id: id,
            name: name,
            version: version,
            base: base,
            image: image,
            logo: logo,
            supports: supports,
            }
    }
}