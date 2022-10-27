use crate::crypto::hash::H256;
use scsys::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockHeader {
    pub id: i64,
    pub hash: H256,
    pub key: String,
    pub nonce: String,
    pub previous: H256,
    pub timestamp: i64,
}

impl BlockHeader {
    pub fn new(id: i64, hash: H256, key: String, nonce: String, previous: H256) -> Self {
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
