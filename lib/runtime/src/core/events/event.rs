/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::Eventful;
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Event {
    pub message: String,
    pub timestamp: Timestamp,
}

impl Event {
    pub fn new(message: String, timestamp: Timestamp) -> Self {
        Self { message, timestamp }
    }
}

impl Eventful for Event {
    fn message(&self) -> String {
        self.message.clone()
    }
    fn timestamp(&self) -> i64 {
        self.clone().timestamp.into()
    }
}

impl std::convert::From<String> for Event {
    fn from(data: String) -> Self {
        Self::new(data, Timestamp::default())
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::from(String::new())
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
