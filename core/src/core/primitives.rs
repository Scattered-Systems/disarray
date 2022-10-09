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
    use scsys::prelude::chrono;

    pub type BlockId = u64;
    pub type BlockHs = String;
    pub type BlockNc = u64;
    pub type BlockTs = i64;
    pub type BlockTz = chrono::Utc;

    /// Simplistic wrapper for implementing transaction data
    pub type BlockData<Dt = String> = Vec<Dt>;
}
