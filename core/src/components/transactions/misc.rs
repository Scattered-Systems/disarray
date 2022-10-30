/*
   Appellation: misc <transactions>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::signed::SignedTransaction;
use scsys::crypto::hash::H256;
use serde::{Deserialize, Serialize};

/// This structure models the expected signature
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sign {
    pub pubk: Vec<u8>,
    pub sig: Vec<u8>,
}

impl Sign {
    pub fn new(pubk: Vec<u8>, sig: Vec<u8>) -> Self {
        Self { pubk, sig }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.pubk, self.sig)
    }
}

/// Implements a spad-id for malicious transactions
#[derive(Eq, Hash, PartialEq, Serialize)]
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
