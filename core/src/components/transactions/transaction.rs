/*
    Appellation: transaction <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        This module implements the structure for a transaction destined to be used within a block on a blockchain
*/
use crate::crypto::hash::{Hashable, H160, H256};
use scsys::prelude::ring;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Transaction {
    pub recv: H160,
    pub value: usize,
    pub nonce: usize,
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        let serialized: Vec<u8> = serde_json::to_vec(self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &serialized).into()
    }
}
