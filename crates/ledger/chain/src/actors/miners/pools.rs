/*
   Appellation: pools <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::SignedTransactions;
use decanter::prelude::H256;
use scsys::prelude::Locked;
use std::convert::From;

#[derive(Clone)]
pub struct Pools {
    pub mempool: Locked<SignedTransactions>,
    pub transpool: Locked<Vec<H256>>,
}

impl Pools {
    pub fn new(mempool: Locked<SignedTransactions>, transpool: Locked<Vec<H256>>) -> Self {
        Self { mempool, transpool }
    }
}

impl Default for Pools {
    fn default() -> Self {
        Self::new(
            Locked::new(Default::default()),
            Locked::new(Default::default()),
        )
    }
}

impl From<(SignedTransactions, Vec<H256>)> for Pools {
    fn from(data: (SignedTransactions, Vec<H256>)) -> Self {
        Self::new(Locked::new(data.0.into()), Locked::new(data.1.into()))
    }
}
