/*
    Appellation: agent <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockFetch;

impl BlockFetch {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BlockFetch {
    fn default() -> Self {
        Self::new()
    }
}
