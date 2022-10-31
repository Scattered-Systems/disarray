/*
   Appellation: interface <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::{
    create_merkle_tree, merkle_proof, MerkleDimension, MerkleShape, MerkleTreeWrapper,
    MerkleTreeWrapperExt,
};

use scsys::crypto::hash::{Hashable, H256};

#[derive(Debug, Default)]
pub struct MerkleTree {
    pub dim: MerkleDimension,
    pub nodes: Vec<H256>,
}

impl MerkleTree {
    pub fn new(dim: MerkleDimension, nodes: Vec<H256>) -> Self {
        Self { dim, nodes }
    }
    pub fn create<T: Hashable>(data: &[T]) -> Self {
        let (dim, nodes) = create_merkle_tree(data);

        Self::new(dim.into(), nodes)
    }
    // Returns the proof for the given index
    pub fn proof(&self, index: usize) -> Vec<H256> {
        merkle_proof(self.dim.clone(), self.nodes.clone(), index)
    }
}

impl MerkleTreeWrapper for MerkleTree {
    fn new(dim: MerkleDimension, nodes: Vec<H256>) -> Self {
        Self { dim, nodes }
    }

    fn dim(&self) -> MerkleDimension {
        self.dim.clone()
    }

    fn nodes(&self) -> Vec<H256> {
        self.nodes.clone()
    }
}

impl MerkleTreeWrapperExt for MerkleTree {}

impl<T: Hashable> std::convert::From<&[T]> for MerkleTree {
    fn from(data: &[T]) -> Self {
        let (dim, nodes) = create_merkle_tree::<T>(data);
        Self::new(MerkleDimension::from(dim), nodes)
    }
}

impl std::convert::From<(Box<dyn MerkleShape>, Vec<H256>)> for MerkleTree {
    fn from(data: (Box<dyn MerkleShape>, Vec<H256>)) -> Self {
        Self::new(MerkleDimension::from(data.0), data.1)
    }
}

impl std::convert::From<Vec<H256>> for MerkleTree {
    fn from(data: Vec<H256>) -> Self {
        Self::from(create_merkle_tree(&data))
    }
}
