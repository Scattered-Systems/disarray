/*
    Appellation: engine <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{CoreEngineSpec, CoreEngineWrapper, CoreEngineWrapperExt};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Engine {
    pub timestamp: i64,
}

impl Engine {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Timestamp::timestamp())
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl CoreEngineSpec for Engine {}

impl CoreEngineWrapper for Engine {}

impl CoreEngineWrapperExt for Engine {}
