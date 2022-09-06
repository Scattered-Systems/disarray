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
    /// Simplistic wrapper for implementing transaction data
    pub type BlockData<Dt = String> = Vec<Dt>;
}
