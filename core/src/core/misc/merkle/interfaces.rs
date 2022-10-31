/*
   Appellation: interfaces <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::{create_merkle_tree, merkle_proof, MerkleDimension, MerkleShape};
use scsys::prelude::{Hashable, H256};

pub trait MerkleTreeWrapper {
    fn new(dim: MerkleDimension, nodes: Vec<H256>) -> Self;
    fn create<T>(data: &[T]) -> Self
    where
        Self: Sized,
        T: Hashable,
    {
        let (dim, nodes) = create_merkle_tree(data);

        Self::new(dim.into(), nodes)
    }
    fn dim(&self) -> MerkleDimension;
    fn nodes(&self) -> Vec<H256>;
    // Returns the proof for the given index
    fn proof(&self, index: usize) -> Vec<H256> {
        merkle_proof(self.dim(), self.nodes(), index)
    }
    // Returns the root hash of the merkle tree
    fn root(&self) -> H256 {
        self.nodes()[self.dim().size() - 1]
    }
}

pub trait MerkleTreeWrapperExt: MerkleTreeWrapper {
    // Writes the injected nodes to the console for viewing purposes
    fn print(&self) -> &Self {
        for i in 0..self.dim().size {
            println!("{:?}", self.nodes()[i]);
        }
        self
    }
}
