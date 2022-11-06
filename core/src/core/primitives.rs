/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{constants::*, types::*};

mod constants {
    /// Set the difficulty for mining new blocks
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {
    use crate::transactions::{SignedTransaction, Transaction};
    use scsys::prelude::{chrono, H160, H256};
    use std::collections::HashMap;

    pub type BlockId = i64;
    pub type BlockHs = H256;
    pub type BlockNc = u32;
    pub type BlockTs = i64;
    pub type BlockTz = chrono::Utc;

    /// Simplistic wrapper for implementing transaction data
    pub type BlockData<Dt = String> = Vec<Dt>;
    pub type BlockState = HashMap<H256, StateMap>;
    /// Type alias for a vector of signed transactions
    pub type SignedTransactions = Vec<SignedTransaction>;
    /// Type alias for a stateful hash map
    pub type StateMap = HashMap<H160, (usize, usize)>;
    /// Type alias for a vector of unsigned transactions
    pub type Transactions = Vec<Transaction>;
}
