/*
    Appellation: content <blocks>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{crypto::{hash::{Hashable, H256}, merkle::MerkleTree}, transactions::SignedTransaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockContent {
    pub data: Vec<SignedTransaction>,
    pub reference: H256,
}

impl BlockContent {
    pub fn new(data: Vec<SignedTransaction>, reference: H256) -> Self {
        Self {
            data,
            reference,
        }
    }
}

impl Hashable for BlockContent {
    fn hash(&self) -> H256 {
        let mt: MerkleTree = MerkleTree::new(&self.data);
        mt.root()
    }
}
