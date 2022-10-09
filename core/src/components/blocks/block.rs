/*
   Appellation: block <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::{create_block_by_mining, transactions::Transactions};
use scsys::prelude::chrono;
use serde::{Deserialize, Serialize};

type BlockId = u64;
type BlockHs = String;
type BlockNc = u64;
type BlockTs = i64;
type BlockTz = chrono::Utc;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub hash: BlockHs,
    pub nonce: BlockNc,
    pub previous: BlockHs,
    pub timestamp: BlockTs,
    pub transactions: Transactions,
}

impl Block {
    pub fn new(id: BlockId, previous: BlockHs, transactions: Transactions) -> Self {
        let timestamp = BlockTz::now().timestamp();
        let (nonce, hash) = create_block_by_mining(
            id.clone(),
            previous.clone(),
            timestamp.clone(),
            transactions.clone(),
        );
        Self {
            id,
            hash,
            nonce,
            previous,
            timestamp,
            transactions,
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block(\nid={},\nhash={},\nnonce={},\nprevious={},\ntimestamp={:#?},\ndata={:#?})",
            self.id, self.hash, self.nonce, self.previous, self.timestamp, self.transactions
        )
    }
}
