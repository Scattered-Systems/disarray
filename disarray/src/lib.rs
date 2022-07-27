/*
   Appellation: disarray <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Disarray is a complete toolkit for building efficient EVM compatible Multi-Chain Networks.
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use disarray_core::*;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use disarray_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use disarray_macros::*;
#[doc(inline)]
#[cfg(feature = "network")]
pub use disarray_network as network;

mod prelude {
    #[cfg(feature = "network")]
    pub use crate::network::*;
}
