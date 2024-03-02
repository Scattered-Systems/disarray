/*
   Appellation: disarray <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Disarray leverages a malleable core building off the component architecture being introduced in the WebAssembley Stack to deliver a high-preformant,
       post-quantum multi-chian supporting both PoS and PoW workloads as acceptable means of generating new blocks. This chain serves as the technical foudnation
       for more rigorous services, Aether and Chaos, implementing optimized surfaces for executing a variety of different commands contexutalized and proxied
       according to the active Flow module.
*/
#[cfg(feature = "ledger")]
pub use disarray_chain as ledger;
#[cfg(feature = "network")]
pub use disarray_network as net;

pub mod prelude {
    pub use super::*;

    #[cfg(feature = "ledger")]
    pub use super::ledger::*;
    #[cfg(feature = "network")]
    pub use super::net::*;
}
