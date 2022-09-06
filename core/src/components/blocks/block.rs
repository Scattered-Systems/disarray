/*
   Appellation: block <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::{transactions::Transactions, create_block_by_mining};
use scsys::{BlockHs, BlockId, BlockNc, BlockTs, BlockTz};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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
        Self { id, hash, nonce, previous, timestamp, transactions }
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

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::{transactions::Transactions, determine_block_validity};

    #[test]
    fn test_block_validity() {
        let pblock = Block::new(0u64, "genesis_block".to_string(),Transactions::new());
        let nblock = Block::new(1u64, pblock.hash.clone(), Transactions::new());
        assert_eq!(determine_block_validity(&nblock, &pblock), true)
    }
}
