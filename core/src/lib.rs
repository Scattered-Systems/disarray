/*
   Appellation: disarray-core <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
    ... Summary ...
*/
#[cfg(test)]
#[macro_use]
extern crate hex_literal;

#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;
