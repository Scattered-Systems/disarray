/*
   Appellation: signed <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::{Sign, Transaction};
use scsys::{prelude::*, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct SignedTransaction {
    pub sig: Sign,
    pub trx: Transaction,
}

impl SignedTransaction {
    pub fn new(sig: Sign, trx: Transaction) -> Self {
        Self { sig, trx }
    }
}

impl std::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
