/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{addressing::*, constants::*, types::*};

pub(crate) mod addressing;

pub(crate) mod constants {
    ///
    pub const MAX_INCOMING_CLIENT: usize = 256;
    ///
    pub const MAX_EVENT: usize = 1024;

    /// Set the difficulty for mining new blocks
    pub const DIFFICULTY_PREFIX: &str = "00";

    pub const INITIAL_POW_DIFFICULTY: [u8; 32] = [
        0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    pub const INITIAL_POS_DIFFICULTY: [u8; 32] = [1; 32];

    pub const MAINNET_PORT: u16 = 9090;
    pub const TESTNET_PORT: u16 = 9091;
}

pub(crate) mod types {
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};

    pub use libp2p::{
        dns::{GenDnsConfig, TokioDnsConfig},
        noise::{AuthenticKeypair, NoiseError},
    };

    /// Type alias for [libp2p::noise::AuthenticKeypair<libp2p::noise::X25519Spec>]
    pub type AuthNoiseKeys = AuthenticKeypair<NoiseSpec>;
    /// Type alias for a boxed transport
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    /// Type alias for [libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>]
    pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
    /// Type alias for [libp2p::Multiaddr]
    pub type NetworkAddress = libp2p::Multiaddr;
    /// Type alias for [libp2p::noise::Keypair<libp2p::noise::X25519Spec>]
    pub type NoiseKeys = libp2p::noise::Keypair<NoiseSpec>;
    ///
    pub type NoiseResult<T = AuthNoiseKeys> = Result<T, NoiseError>;
    /// Type alias for [libp2p::noise::X25519Spec]
    pub type NoiseSpec = libp2p::noise::X25519Spec;
    /// Type alias for [libp2p::PeerId]
    pub type PeerId = libp2p::PeerId;
    /// Type alias for [libp2p::identity::Keypair]
    pub type PeerKp = libp2p::identity::Keypair;
    /// Type alias for
    pub type P2PTransport = (PeerId, libp2p::core::muxing::StreamMuxerBox);
}
