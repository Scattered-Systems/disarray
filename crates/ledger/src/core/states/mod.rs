/*
    Appellation: states <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::state::*;

pub(crate) mod state;

use crate::BlockState;

pub trait BlockchainStateWrapper {
    fn blockstate(&self) -> BlockState;
}
