/*
   Appellation: common
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {}

mod types {
     pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
}
