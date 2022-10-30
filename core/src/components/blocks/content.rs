/*
    Appellation: content <blocks>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::merkle::MerkleTree;
use crate::transactions::SignedTransaction;
use scsys::crypto::hash::{Hashable, H256};
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
        let mt: MerkleTree = MerkleTree::new(&self.data);
        mt.root()
    }
}
