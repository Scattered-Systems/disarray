/*
   Appellation: signed <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::Transaction;
use scsys::crypto::hashes::{H256, Hashable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Signature {
    pub public_key: Vec<u8>,
    pub sig: Vec<u8>
}

impl Signature {
    pub fn new(public_key: Vec<u8>, sig: Vec<u8>) -> Self {
        Self { public_key, sig }
    }
}
impl Default for Signature {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Signature
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, signature: Signature) -> Self {
        Self { transaction, signature }
    }
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
        let serialized: Vec<u8> = bincode::serialize(&self).unwrap();
        let bytes: &[u8] = &serialized;
        ring::digest::digest(&ring::digest::SHA256, bytes).into()
    }
}