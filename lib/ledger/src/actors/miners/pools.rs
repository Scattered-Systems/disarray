/*
   Appellation: pools <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{Lock, SignedTransactions};
use scsys::prelude::H256;
use std::convert::From;

#[derive(Clone)]
pub struct Pools {
    pub mempool: Lock<SignedTransactions>,
    pub transpool: Lock<Vec<H256>>,
}

impl Pools {
    pub fn new(mempool: Lock<SignedTransactions>, transpool: Lock<Vec<H256>>) -> Self {
        Self { mempool, transpool }
    }
}

impl Default for Pools {
    fn default() -> Self {
        Self::new(Lock::new(Default::default()), Lock::new(Default::default()))
    }
}

impl From<(SignedTransactions, Vec<H256>)> for Pools {
    fn from(data: (SignedTransactions, Vec<H256>)) -> Self {
        Self::new(Lock::new(data.0), Lock::new(data.1))
    }
}
