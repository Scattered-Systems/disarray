/*
    Appellation: agent <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChainSync;

impl ChainSync {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ChainSync {
    fn default() -> Self {
        Self::new()
    }
}
