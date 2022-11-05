/*
   Appellation: misc <transactions>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::SignedTransaction;
use scsys::crypto::hash::H256;
use serde::{Deserialize, Serialize};

/// Implements a spad-id for malicious transactions
#[derive(Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SpamId {
    pub nonce: usize,
    pub pubk: String,
}

impl SpamId {
    pub fn new(nonce: usize, pubk: String) -> Self {
        Self { nonce, pubk }
    }
}

impl std::convert::From<&SignedTransaction> for SpamId {
    fn from(t: &SignedTransaction) -> Self {
        let hash: H256 = t.sign.pubk.clone().into();
        SpamId {
            nonce: t.transaction.nonce,
            pubk: hash.to_string(),
        }
    }
}
