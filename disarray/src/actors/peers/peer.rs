/*
   Appellation: peer <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub type PeerId = libp2p::PeerId;
pub type PeerKP = libp2p::identity::Keypair;

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: PeerKP,
}

impl Peer {
    pub fn constructor(id: PeerId, key: PeerKP) -> Self {
        Self { id, key }
    }
    pub fn new() -> Self {
        let key = PeerKP::generate_ed25519();
        let id = PeerId::from(&key.public());
        Self::constructor(id.clone(), key.clone())
    }
    pub fn from(key: PeerKP) -> Self {
        Self::constructor(PeerId::from(&key.public()), key)
    }
    pub fn authenticate(&self) -> crate::AuthNoiseKeys {
        crate::create_auth_noise_keys(&self.key)
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(\nid={:#?},\nkey={:#?}\n)", self.id, self.key)
    }
}
