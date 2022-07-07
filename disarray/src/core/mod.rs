/*
   Appellation: core
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::core::common::*;

mod common {
    pub use constants::*;
    pub use types::*;
    pub use variants::*;

    mod constants {

        pub const DIFFICULTY_PREFIX: &str = "00";
    }

    mod types {}

    mod variants {
        pub enum Dates {
            Standard(i64),
        }
    }
}
