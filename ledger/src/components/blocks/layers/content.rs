/*
   Appellation: content <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::SignedTransaction;
use algae::merkle::{MerkleTree, MerkleTreeWrapper};
use scsys::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

impl std::fmt::Display for BlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
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
