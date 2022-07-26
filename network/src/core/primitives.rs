/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    /// Define the port typically associated with a mainnet
    pub const MAINNET_PORT: u16 = 9090;
    /// Define the port typically associated with a testnet
    pub const TESTNET_PORT: u16 = 9091;
}

mod types {
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};

    /// Wrapper for libp2p::noise::AuthenticKeypair<libp2p::noise::X25519Spec>
    pub type AuthNoiseKeys = libp2p::noise::AuthenticKeypair<NoiseSpec>;
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
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
