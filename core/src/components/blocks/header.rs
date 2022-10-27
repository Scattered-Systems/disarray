/*
   Appellation: header <blocks>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::crypto::hash::{hasher, Hashable, H256};
use scsys::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockHeader {
    pub id: i64,
    pub hash: H256,
    pub key: String,
    pub nonce: usize,
    pub previous: H256,
    pub timestamp: i64,
}

impl BlockHeader {
    pub fn new(id: i64, hash: H256, key: String, nonce: usize, previous: H256) -> Self {
        let timestamp = Timestamp::timestamp();
        Self {
            id,
            hash,
            key,
            nonce,
            previous,
            timestamp,
        }
    }
}

impl Hashable for BlockHeader {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}
