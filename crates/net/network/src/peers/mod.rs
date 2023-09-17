/*
    Appellation: peers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::peer::*;

pub(crate) mod peer;

use crate::{authorize_peer, generate_noise_keys};
use crate::{AuthNoiseKeys, NoiseKeys, NoiseResult, PeerId, PeerKp};

pub trait PeerWrapper {
    fn id(&self) -> PeerId;
    fn keypair(&self) -> PeerKp;
    fn authenticate(&self, noise_keys: NoiseKeys) -> NoiseResult<AuthNoiseKeys> {
        authorize_peer(noise_keys, self.keypair())
    }
    fn generate_ed25519() -> PeerKp {
        PeerKp::generate_ed25519()
    }
    fn generate_noise_keys() -> NoiseKeys {
        generate_noise_keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_peer() {
        let kp = Peer::generate_ed25519();
        let nk = Peer::generate_noise_keys();
        let peer = Peer::from(kp);
        let a = peer.authenticate(nk.clone());
        let b = authorize_peer(nk, peer.keypair());
        assert!(a.is_ok());
        assert!(b.is_ok());
    }
}
