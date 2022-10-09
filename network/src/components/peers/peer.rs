/*
   Appellation: peer <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{AuthNoiseKeys, NoiseError, NoiseKeys, PeerId, PeerKp};
use std::borrow::Borrow;

/// Implement a standard peer on a p2p network
#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub keypair: PeerKp,
}

impl Peer {
    fn generate_noise_keys() -> NoiseKeys {
        crate::NoiseKeys::new()
    }
    pub fn authenticate(self) -> Result<AuthNoiseKeys, NoiseError> {
        Self::generate_noise_keys().into_authentic(self.keypair.clone().borrow())
    }
    pub fn new(id: PeerId, keypair: PeerKp) -> Self {
        Self { id, keypair }
    }
}

impl std::convert::From<PeerKp> for Peer {
    fn from(data: PeerKp) -> Self {
        Self::new(PeerId::from(data.clone().public()), data)
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::from(PeerKp::generate_ed25519())
    }
}
