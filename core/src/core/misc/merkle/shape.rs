/*
   Appellation: shape <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

pub trait MerkleShape {
    fn depth(&self) -> usize;
    fn leafs(&self) -> usize;
    fn shape(&self) -> (usize, usize, usize) {
        (self.depth(), self.leafs(), self.size())
    }
    fn size(&self) -> usize;
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleDimension {
    pub depth: usize,
    pub leafs: usize,
    pub size: usize,
}

impl MerkleDimension {
    pub fn new(depth: usize, leafs: usize, size: usize) -> Self {
        Self { depth, leafs, size }
    }
}

impl MerkleShape for MerkleDimension {
    fn depth(&self) -> usize {
        self.depth
    }

    fn leafs(&self) -> usize {
        self.leafs
    }
    fn size(&self) -> usize {
        self.size
    }
}

impl std::convert::From<Box<dyn MerkleShape>> for MerkleDimension {
    fn from(data: Box<dyn MerkleShape>) -> Self {
        Self::from(data.shape())
    }
}

impl std::convert::From<(usize, usize, usize)> for MerkleDimension {
    fn from(data: (usize, usize, usize)) -> Self {
        Self::new(data.0, data.1, data.2)
    }
}

impl std::fmt::Display for MerkleDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.depth, self.leafs, self.size)
    }
}
