/*
   Appellation: block <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::create_block_by_mining;
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
        let (nonce, hash) = create_block_by_mining(
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

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::determine_block_validity;

    #[test]
    fn test_block_validity() {
        let transactions = Vec::<String>::new();
        let pblock = Block::new(1u64, "genesis_block".to_string(), transactions.clone());
        let nblock = Block::new(2u64, pblock.hash.clone(), transactions.clone());
        assert_eq!(determine_block_validity(&nblock, &pblock), true)
    }
}
