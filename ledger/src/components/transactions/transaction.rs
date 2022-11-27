/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::validate_transaction_signature;
use ring::signature::{self, Ed25519KeyPair};
use scsys::{prelude::*, Hashable, };
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
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

    #[test]
    fn test_transaction_default() {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).ok().unwrap();
        let kp = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).ok().unwrap();
        let a = Transaction::default();
        let sig = a.sign(&kp);
        assert!(a.validate(&kp, &sig))
    }
}