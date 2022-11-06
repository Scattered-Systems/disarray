/*
    Appellation: chain_data <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockData {
    pub block: Block,
    pub height: u128,
}

impl BlockData {
    pub fn new(block: Block, height: u128) -> Self {
        Self { block, height }
    }
}
