/*
    Appellation: peers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{peer::*, specs::*, utils::*};

pub(crate) mod peer;

pub(crate) mod specs {
    use super::{authorize_peer, generate_noise_keys};
    use crate::{AuthNoiseKeys, NoiseKeys, NoiseResult, PeerId, PeerKp};

    pub trait PeerSpec {
        fn id(&self) -> PeerId;
        fn keypair(&self) -> PeerKp;
    }

    pub trait PeerWrapper: PeerSpec {
        fn authenticate(&self, noise_keys: NoiseKeys) -> NoiseResult<AuthNoiseKeys> {
            authorize_peer(noise_keys, self.keypair())
        }
    }

    pub trait PeerWrapperExt: PeerWrapper {
        fn generate_ed25519() -> PeerKp {
            PeerKp::generate_ed25519()
        }
        fn generate_noise_keys() -> NoiseKeys {
            generate_noise_keys()
        }
    }
}

pub(crate) mod utils {
    use crate::{NoiseKeys, NoiseResult, PeerKp};

    pub fn authorize_peer(noise_keys: NoiseKeys, kp: PeerKp) -> NoiseResult {
        noise_keys.into_authentic(&kp)
    }

    pub fn generate_noise_keys() -> NoiseKeys {
        NoiseKeys::new()
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
