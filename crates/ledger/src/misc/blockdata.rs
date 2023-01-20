/*
    Appellation: chain_data <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::blocks::Block;
use decanter::prelude::{Hash, Hashable};
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

impl std::fmt::Display for BlockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{generate_random_block, BlockType};

    #[test]
    pub fn test_blockdata_defaults() {
        let block = generate_random_block(BlockType::PoS, true);
        let a = BlockData::new(block.clone(), 1);
        let b = BlockData::new(block, 1);
        assert_eq!(&a, &b);
    }
}
