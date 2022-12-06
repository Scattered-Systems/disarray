/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...

    TODO:
        - Peer { id, keypair } Find a more convienent type aliases for storing and operability
*/
use crate::{
    peers::{PeerSpec, PeerWrapper, PeerWrapperExt},
    PeerId, PeerKp,
};

/// Implements the peer structure for the network
#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub keypair: PeerKp,
}

impl Peer {
    pub fn new(id: PeerId, keypair: PeerKp) -> Self {
        Self { id, keypair }
    }
}

impl PeerSpec for Peer {
    fn id(&self) -> PeerId {
        self.id
    }
    fn keypair(&self) -> PeerKp {
        self.keypair.clone()
    }
}

impl PeerWrapper for Peer {}

impl PeerWrapperExt for Peer {}

impl std::convert::From<&[u8]> for Peer {
    fn from(data: &[u8]) -> Self {
        Self::from(PeerKp::from_protobuf_encoding(data).expect(""))
    }
}

impl std::convert::From<PeerKp> for Peer {
    fn from(data: PeerKp) -> Self {
        Self::new(PeerId::from(data.public()), data)
    }
}

impl std::convert::From<(PeerId, PeerKp)> for Peer {
    fn from(data: (PeerId, PeerKp)) -> Self {
        Self::new(data.0, data.1)
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::from(PeerKp::generate_ed25519())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_peer() {
        let kp = PeerKp::generate_ed25519();
        let a = Peer::from(kp.clone());
        let b = Peer::new(PeerId::from(kp.public()), kp);
        assert!(a.id == b.id);
    }
}
