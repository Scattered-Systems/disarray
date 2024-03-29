/*
   Appellation: misc <transactions>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::SignedTransaction;
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

/// Implements a spad-id for malicious transactions
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SpamId {
    pub nonce: usize,
    pub pubk: String,
}

impl SpamId {
    pub fn new(nonce: usize, pubk: String) -> Self {
        Self { nonce, pubk }
    }
}

impl Hashable for SpamId {
    fn hash(&self) -> H256 {
        hasher(&self).into()
    }
}

impl std::convert::From<&SignedTransaction> for SpamId {
    fn from(t: &SignedTransaction) -> Self {
        let hash: H256 = t.sig.pubk.clone().into();
        SpamId {
            nonce: t.trx.nonce,
            pubk: hash.to_string(),
        }
    }
}

impl std::fmt::Display for SpamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
