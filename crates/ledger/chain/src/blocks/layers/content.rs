/*
   Appellation: content <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::SignedTransaction;
use algae::merkle::{MerkleTree, MerkleTreeWrapper};
use decanter::prelude::{Hashable, H256};
use scsys::SerdeDisplay;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, SerdeDisplay, Serialize)]
pub struct BlockContent {
    pub data: Vec<SignedTransaction>,
    pub reference: Vec<H256>,
}

impl BlockContent {
    pub fn new(data: Vec<SignedTransaction>, reference: Vec<H256>) -> Self {
        Self { data, reference }
    }
}

impl Hashable for BlockContent {
    fn hash(&self) -> H256 {
        let mt = MerkleTree::create(&self.data);
        mt.root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_block_content() {
        let a = BlockContent::default();
        let b = BlockContent::new(Default::default(), Default::default());
        assert_eq!(&a, &b);
    }
}
