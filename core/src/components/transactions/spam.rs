/*
   Appellation: spam <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::crypto::hash::H256;
use super::signed::SignedTransaction;
use serde::Serialize;

#[derive(Eq, Hash, PartialEq, Serialize)]
pub struct SpamId {
    pub nonce: usize,
    pub pubk: String,
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