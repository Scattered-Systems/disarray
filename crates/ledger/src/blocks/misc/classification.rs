/*
    Appellation: classification <blocks>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use decanter::prelude::{Hash, Hashable,};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BlockType {
    PoS,
    #[default]
    PoW,
}

impl std::convert::From<bool> for BlockType {
    fn from(data: bool) -> Self {
        match data {
            true => Self::PoS,
            false => Self::PoW,
        }
    }
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap().to_lowercase()
        )
    }
}
