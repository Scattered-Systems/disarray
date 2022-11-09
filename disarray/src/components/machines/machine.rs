/*
    Appellation: machine <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{BoxResult, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]

pub struct Session {
    pub started: Timestamp
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine {
    pub commands: HashMap<String, Value>,
    pub data: Vec<Value>,
    pub results: Vec<String>,
    pub session: Session
}


impl Machine {
    pub async fn runner(&mut self) -> BoxResult {
        Ok(())
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
