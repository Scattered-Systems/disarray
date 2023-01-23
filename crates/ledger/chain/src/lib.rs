/*
   Appellation: disarray-ledger <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Disarray is a complete toolkit for building efficient EVM compatible Multi-Chain Networks.
*/
// #[cfg(test)]
extern crate hex_literal;

pub use disarray_consensus as consensus;

#[doc(inline)]
pub use crate::{actors::*, blockchain::*, misc::*, primitives::*, utils::*};

pub(crate) mod actors;
pub(crate) mod blockchain;
pub(crate) mod misc;
pub(crate) mod primitives;
pub(crate) mod utils;

pub mod blocks;
pub mod clients;
pub mod handles;
pub mod proofs;
pub mod states;
pub mod transactions;
