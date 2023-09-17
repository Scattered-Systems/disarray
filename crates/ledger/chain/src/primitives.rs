/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    /// Set the difficulty for mining new blocks
    pub const DIFFICULTY_PREFIX: &str = "00";

    pub const INITIAL_POW_DIFFICULTY: [u8; 32] = [
        0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    pub const INITIAL_POS_DIFFICULTY: [u8; 32] = [1; 32];
}

pub(crate) mod types {
    use crate::transactions::{SignedTransaction, Transaction};
    use crate::{BlockStore, Merger};
    use ckb_merkle_mountain_range::MMR;
    use decanter::prelude::{H160, H256};
    use std::collections::HashMap;

    /// Type alias for a block id; [i64]
    pub type BlockId = i64;
    /// Type alias for a block hash; [H256]
    pub type BlockHs = H256;
    /// Type alias for an block's nonce; [u32]
    pub type BlockNc = u32;
    /// Type alias for a block's timestamp; [i64]
    pub type BlockTs = i64;
    /// Type alias for a block's timezone; [chrono::Utc]
    pub type BlockTz = chrono::Utc;
    /// Type alias for a block state; [HashMap<H256, StateMap>]
    pub type BlockState = HashMap<H256, StateMap>;
    /// Type alias for a blockchain merkle mountain range; [MMR]
    pub type ChainMMR = MMR<H256, Merger, BlockStore<H256>>;
    /// Type alias for a vector of signed transactions; [Vec<SignedTransaction>]
    pub type SignedTransactions = Vec<SignedTransaction>;
    /// Type alias for a stateful hash map; [HashMap<H160, (usize, usize)>]
    pub type StateMap = HashMap<H160, (usize, usize)>;
    /// Type alias for a vector of unsigned transactions; [Vec<Transaction>]
    pub type Transactions = Vec<Transaction>;
}
