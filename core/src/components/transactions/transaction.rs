/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        This module implements the structure for a transaction destined to be used within a block on a blockchain
*/
use scsys::prelude::{ring::signature::{Ed25519KeyPair, Signature}, H160, H256, Hashable, hasher};
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

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {:?}, {})", self.nonce, self.recv.0, self.value)
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        hasher(self).as_slice().to_owned().into()
    }
}
