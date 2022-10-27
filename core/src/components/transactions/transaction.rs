/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        This module implements the structure for a transaction destined to be used within a block on a blockchain
*/
use crate::crypto::hash::{Hashable, H160, H256, hasher};
use scsys::prelude::ring::{
    self,
    signature::{Ed25519KeyPair, Signature},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Transaction {
    pub nonce: usize,
    pub recv: H160,
    pub value: usize,
}

impl Transaction {
    pub fn new(nonce: usize, recv: H160, value: usize) -> Self {
        Self { nonce, recv, value }
    }
    /// Create digital signature of a transaction
    pub fn sign(&self, key: &Ed25519KeyPair) -> Signature {
        key.sign(&serde_json::to_vec(self).unwrap())
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}
