/*
   Appellation: layers <merkle>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::nodes::MerkleNode;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

// pub fn build_new_merkle_layer<T: ToString>(left: MerkleNode<T>, right: MerkleNode)
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleLayer<T: ToString>(Vec<MerkleNode<T>>);

impl<T: ToString> MerkleLayer<T> {
    pub fn new(data: Vec<MerkleNode<T>>) -> Self {
        let layer = data.into_iter().batching(|it| match it.next() {
            Some(l) => match it.next() {
                Some(r) => Some(MerkleNode::from((l, r))),
                None => Some(l),
            },
            None => None,
        });

        Self(layer.collect::<Vec<_>>())
    }
}

impl<T: ToString> std::convert::Into<Vec<MerkleNode<T>>> for MerkleLayer<T> {
    fn into(self) -> Vec<MerkleNode<T>> {
        self.0
    }
}
