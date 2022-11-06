/*
    Appellation: classification <blocks>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
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
