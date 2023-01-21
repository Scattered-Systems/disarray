/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{sign, validate_transaction_signature};
use decanter::prelude::{Hash, Hashable, H160};
use ring::signature::{self, Ed25519KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
        sign(key, &self)
    }
    /// Validate the transaction given the correct key, signature pair
    pub fn validate(&self, key: &Ed25519KeyPair, sig: &signature::Signature) -> bool {
        validate_transaction_signature(self, key, sig)
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::prelude::random_keypair;

    #[test]
    fn test_transaction_default() {
        let kp = random_keypair();
        let a = Transaction::default();
        let sig = a.sign(&kp);
        assert!(a.validate(&kp, &sig))
    }
}
