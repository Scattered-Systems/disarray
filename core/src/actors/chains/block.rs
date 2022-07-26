/*
   Appellation: block <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::{BlockHs, BlockId, BlockNc, BlockTs, BlockTz};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Block<Dt: Clone + serde::Serialize = String> {
    pub id: BlockId,
    pub hash: BlockHs,
    pub nonce: BlockNc,
    pub previous: BlockHs,
    pub timestamp: BlockTs,
    pub transactions: Vec<Dt>,
}

impl<Dt: Clone + serde::Serialize> Block<Dt> {
    pub fn constructor(
        id: BlockId,
        hash: BlockHs,
        nonce: BlockNc,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<Dt>,
    ) -> Self {
        Self {
            id,
            hash,
            nonce,
            previous,
            timestamp,
            transactions,
        }
    }
    pub fn new(id: BlockId, previous: BlockHs, transactions: Vec<Dt>) -> Self {
        let timestamp = BlockTz::now().timestamp();
        let (nonce, hash) = crate::chains::create_block_by_mining(
            id.clone(),
            previous.clone(),
            timestamp.clone(),
            transactions.clone(),
        );
        Self::constructor(
            id.clone(),
            hash.clone(),
            nonce.clone(),
            previous.clone(),
            timestamp.clone(),
            transactions.clone(),
        )
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
