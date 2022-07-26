/*
   Appellation: peer <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{PeerId, PeerKp};

/// Implement a standard peer on a p2p network
#[derive(Clone, Debug)]
pub struct StandardPeer {
    pub id: PeerId,
    pub keypair: PeerKp,
}

impl StandardPeer {
    pub fn new() -> Self {
        let keypair = PeerKp::generate_ed25519();
        let id = PeerId::from(keypair.clone().public());
        Self { id, keypair }
    }
}
