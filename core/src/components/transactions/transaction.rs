/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{
    hasher,
    ring::signature::{Ed25519KeyPair, Signature},
    Hashable, H160, H256,
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

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.nonce, H256::from(self.recv.0), self.value)
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        hasher(self).as_slice().to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_transaction() {
        let a = Transaction::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
