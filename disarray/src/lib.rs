/*
   Appellation: concision
   Context: library
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Concision is a robust framework for creating powerful data-centric applications in Rust.
*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use crate::{actors::*, clients::*, core::*, data::*};
#[doc(inline)]
#[cfg(feature = "core")]
pub use disarray_core::*;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use disarray_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use disarray_macros::*;

mod actors;
mod clients;
mod core;
mod data;
