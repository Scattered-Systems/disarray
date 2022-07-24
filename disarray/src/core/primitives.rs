/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Timestamps {
    Binary(bson::DateTime),
    Standard(scsys::BlockTs),
}

mod constants {
    /// Set the difficulty for mining new blocks
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {
    /// Wrapper for libp2p::noise::AuthenticKeypair<libp2p::noise::X25519Spec>
    pub type AuthNoiseKeys = libp2p::noise::AuthenticKeypair<NoiseSpec>;
    /// Simplistic wrapper for implementing transaction data
    pub type BlockData<Dt = String> = Vec<Dt>;
    /// Wrapper for libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>
    pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
    /// Wrapper for libp2p::Multiaddr
    pub type NetworkAddress = libp2p::Multiaddr;
    /// Wrapper for libp2p::noise::Keypair<libp2p::noise::X25519Spec>
    pub type NoiseKeys = libp2p::noise::Keypair<NoiseSpec>;
    /// Wrapper for libp2p::noise::X25519Spec
    pub type NoiseSpec = libp2p::noise::X25519Spec;
    /// Wrapper for libp2p::PeerId
    pub type PeerId = libp2p::PeerId;
    /// Wrapper for libp2p::identity::Keypair
    pub type PeerKp = libp2p::identity::Keypair;
}
