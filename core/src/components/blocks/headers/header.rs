/*
   Appellation: header <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{BlockNc, BlockTs};
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockHeader {
    pub merkle_root: H256,
    pub nonce: BlockNc,
    pub parent: H256,
    pub pos_difficulty: H256,
    pub pow_difficulty: H256,
    pub rand: u128, // randomness for PoS leader election. TODO: update rand every epoch
    pub timestamp: BlockTs,
    pub vrf_hash: Vec<u8>,
    pub vrf_proof: Vec<u8>,
    pub vrf_pub_key: Vec<u8>,
}

impl BlockHeader {
    pub fn new(
        merkle_root: H256,
        nonce: BlockNc,
        parent: H256,
        pos_difficulty: H256,
        pow_difficulty: H256,
        rand: u128,
        timestamp: i64,
        vrf_hash: Vec<u8>,
        vrf_proof: Vec<u8>,
        vrf_pub_key: Vec<u8>,
    ) -> Self {
        Self {
            merkle_root,
            nonce,
            parent,
            pos_difficulty,
            pow_difficulty,
            rand,
            timestamp,
            vrf_hash,
            vrf_proof,
            vrf_pub_key,
        }
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
