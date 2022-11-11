/*
    Appellation: session <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    pub cache: Vec<Value>,
    pub timestamp: Timestamp,
}

impl Session {
    pub fn new() -> Self {
        let cache = Default::default();
        let timestamp = Timestamp::default();
        Self { cache, timestamp }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap().to_lowercase()
        )
    }
}
