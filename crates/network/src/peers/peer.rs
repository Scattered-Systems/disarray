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
use libp2p::identity::ed25519;

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

impl From<&[u8]> for Peer {
    fn from(data: &[u8]) -> Self {
        Self::from(PeerKp::from_protobuf_encoding(data).expect(""))
    }
}

impl From<PeerKp> for Peer {
    fn from(data: PeerKp) -> Self {
        Self::new(PeerId::from(data.public()), data)
    }
}

impl From<(PeerId, PeerKp)> for Peer {
    fn from(data: (PeerId, PeerKp)) -> Self {
        Self::new(data.0, data.1)
    }
}

impl TryFrom<u8> for Peer {
    type Error = scsys::BoxError;

    fn try_from(seed: u8) -> Result<Peer, Self::Error> {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        if let Ok(sk) = ed25519::SecretKey::from_bytes(&mut bytes) {
            let keypair = PeerKp::Ed25519(sk.into());
            Ok(Peer::from(keypair))
        } else {
            panic!("Failed to find a valid keypair from the provided seed...")
        }
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
