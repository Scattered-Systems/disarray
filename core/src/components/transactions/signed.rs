/*
   Appellation: signed <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::{misc::Sign, transaction::Transaction};
use scsys::{
    crypto::hashes::{Hashable, H256},
    prelude::ring,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SignedTransaction {
    pub sign: Sign,
    pub transaction: Transaction,
    
}

impl SignedTransaction {
    pub fn new(sign: Sign, transaction: Transaction) -> Self {
        Self { sign, transaction }
    }
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
        let serialized: Vec<u8> = serde_json::to_vec(self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &serialized).into()
    }
}
