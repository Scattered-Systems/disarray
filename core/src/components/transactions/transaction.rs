/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::validate_transaction_signature;
use scsys::prelude::{
    hasher,
    ring::signature::{self, ED25519, Ed25519KeyPair, KeyPair, UnparsedPublicKey},
    Hashable, H160, H256, Keypair
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
    pub fn sign(&self, key: &Ed25519KeyPair) -> signature::Signature {
        key.sign(self.to_string().as_bytes())
    }
    /// Validate the transaction given the correct key, signature pair
    pub fn validate(&self, key: &Ed25519KeyPair, sig: &signature::Signature) -> bool {
        validate_transaction_signature(&self, key, sig)
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
