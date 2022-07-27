/*
   Appellation: peer <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{PeerId, PeerKp};
use std::borrow::Borrow;

/// Implement a standard peer on a p2p network
#[derive(Clone, Debug)]
pub struct StandardPeer {
    pub id: PeerId,
    pub keypair: PeerKp,
}

impl StandardPeer {
    fn generate_noise_keys() -> crate::NoiseKeys {
        crate::NoiseKeys::new()
    }
    pub fn authenticate(self) -> Result<crate::AuthNoiseKeys, crate::NoiseError> {
        Self::generate_noise_keys().into_authentic(self.keypair.clone().borrow())
    }
    pub fn new() -> Self {
        let keypair = PeerKp::generate_ed25519();
        let id = PeerId::from(keypair.clone().public());
        Self { id, keypair }
    }
}
