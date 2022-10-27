use scsys::Timestamp;
use serde::{Deserialize, Serialize};
use crate::{BlockId, BlockHs, BlockNc, BlockTs};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockHeader {
    pub id: BlockId,
    pub key: String,
    pub nonce: BlockNc,
    pub previous: BlockHs,
    pub timestamp: BlockTs,
}

impl BlockHeader {
    pub fn new(id: BlockId, key: String, nonce: BlockNc, previous: BlockHs) -> Self {
        let timestamp = Timestamp::timestamp();
        Self { id, key, nonce, previous, timestamp }
    }
}