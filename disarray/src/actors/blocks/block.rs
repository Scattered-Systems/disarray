/*
   Appellation: block
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

use crate::{BlockId, BlockHash, BlockNonce, BlockData, BlockTs, BlockTz};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Block {
    pub id: BlockId,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: BlockTs,
    pub data: BlockData,
}

impl Block {
    pub fn constructor(
        id: BlockId,
        hash: BlockHash,
        nonce: BlockNonce,
        previous: BlockHash,
        timestamp: BlockTs,
        data: BlockData,
    ) -> Self {
        Self {
            id,
            hash,
            nonce,
            previous,
            timestamp,
            data,
        }
    }
    pub fn new(id: BlockId, previous: BlockHash, data: BlockData) -> Self {
        let timestamp = BlockTz::now().timestamp();
        let (nonce, hash) = crate::create_block_by_mining(
            id.clone(), previous.clone(), timestamp.clone(), data.clone(),
        );
        Self::constructor(
            id.clone(),
            hash.clone(),
            nonce.clone(),
            previous.clone(),
            timestamp.clone(),
            data.clone(),
        )
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block(\nid={},\nhash={},\nnonce={},\nprevious={},\ntimestamp={:#?},\ndata={:#?})",
            self.id, self.hash, self.nonce, self.previous, self.timestamp, self.data
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let id: BlockId = 1;
        let data: BlockData = Vec::new();
        let previous: BlockHash = "genesis_block".to_string();
        let block = Block::new(id, previous.clone(), data.clone());
        println!("{:#?}", &block);
        assert_eq!(&block, &block)
    }
}
