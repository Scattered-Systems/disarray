/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::BlockTs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Epoch {
    pub size: u128,
    pub time: BlockTs,
}

impl Epoch {
    pub fn new(size: u128, time: BlockTs) -> Self {
        Self { size, time }
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new(400, 120_000_000)
    }
}
