/*
   Appellation: header <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{
    blocks::{BlockDifficulty, BlockHeaderSpec, BlockJustification, Resistable, Verifiable},
    BlockNc, BlockTs,
};
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockHeader {
    pub difficulty: BlockDifficulty,
    pub justification: BlockJustification,
    pub nonce: BlockNc,
    pub parent: H256,
    pub rand: u128, // randomness for PoS leader election. TODO: update rand every epoch
    pub root: H256,
    pub timestamp: BlockTs,
}

impl BlockHeader {
    pub fn new(
        difficulty: BlockDifficulty,
        justification: BlockJustification,
        nonce: BlockNc,
        parent: H256,
        rand: u128,
        root: H256,
        timestamp: i64,
    ) -> Self {
        Self {
            difficulty,
            justification,
            nonce,
            parent,
            rand,
            root,
            timestamp,
        }
    }
}

impl Resistable for BlockHeader {
    fn pos_difficulty(&self) -> H256 {
        self.difficulty.pos_difficulty()
    }
    fn pow_difficulty(&self) -> H256 {
        self.difficulty.pow_difficulty()
    }
}

impl Verifiable for BlockHeader {
    fn vrf_hash(&self) -> Vec<u8> {
        self.justification.vrf_hash()
    }
    fn vrf_proof(&self) -> Vec<u8> {
        self.justification.vrf_proof()
    }
    fn vrf_pub_key(&self) -> Vec<u8> {
        self.justification.vrf_pub_key()
    }
}

impl BlockHeaderSpec for BlockHeader {
    /// Function wrapper for the block's merkle root
    fn merkle_root(&self) -> H256 {
        self.root
    }
    fn nonce(&self) -> BlockNc {
        self.nonce
    }
    fn parent(&self) -> H256 {
        self.parent
    }
    // randomness for PoS leader election. TODO: update rand every epoch
    fn rand(&self) -> u128 {
        self.rand
    }
    fn timestamp(&self) -> BlockTs {
        self.timestamp
    }
}

impl Hashable for BlockHeader {
    fn hash(&self) -> H256 {
        hasher(&serde_json::to_string(&self).unwrap())
            .as_slice()
            .to_owned()
            .into()
    }
}

impl std::fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {}",
            self.nonce(),
            self.parent(),
            self.merkle_root(),
            self.pos_difficulty(),
            self.pow_difficulty(),
            self.rand(),
            self.timestamp()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blocks::generate_random_block_header, transactions::generate_random_signed_transaction,
    };

    #[test]
    fn test_default_block_header() {
        let a: BlockHeader =
            generate_random_block_header(vec![generate_random_signed_transaction()]);
        let b = a.clone();
        assert_eq!(&a, &b);
    }
}
