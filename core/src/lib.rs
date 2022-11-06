/*
   Appellation: disarray-core <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
    ... Summary ...
*/
// #[cfg(test)]
extern crate hex_literal;

#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;
