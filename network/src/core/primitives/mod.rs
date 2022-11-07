/*
    Appellation: misc <proviers>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{addressing::*, constants::*, statics::*, types::*};

pub(crate) mod addressing;

pub(crate) mod constants {
    /// Define the port typically associated with a mainnet
    pub const MAINNET_PORT: u16 = 9090;
    /// Define the port typically associated with a testnet
    pub const TESTNET_PORT: u16 = 9091;
}

pub(crate) mod statics {

    // lazy_static::lazy_static! {
    //     /// Mainnet config for mina p2p network
    //     pub static ref MAINNET_CONFIG : TransportConfig<'static> = TransportConfig
    //     {
    //         rendezvous_string: b"/coda/0.0.1/5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1",
    //         ..Default::default()
    //     };
    // }
}

pub(crate) mod types {
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};
    pub use libp2p::{
        dns::{GenDnsConfig, TokioDnsConfig},
        noise::{AuthenticKeypair as AuthenticNoiseKeypair, NoiseError},
        tcp::{GenTcpConfig, GenTcpTransport, TokioTcpTransport},
    };

    /// Type alias for [libp2p::noise::AuthenticKeypair<libp2p::noise::X25519Spec>]
    pub type AuthNoiseKeys = AuthenticNoiseKeypair<NoiseSpec>;
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
