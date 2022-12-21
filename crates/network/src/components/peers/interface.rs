/*
    Appellation: interface <peer>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{AuthNoiseKeys, NoiseKeys, NoiseResult, PeerId, PeerKp};

pub trait PeerSpec {
    fn id(&self) -> PeerId;
    fn keypair(&self) -> PeerKp;
}

pub trait PeerWrapper: PeerSpec {
    fn authenticate(&self, noise_keys: NoiseKeys) -> NoiseResult<AuthNoiseKeys> {
        noise_keys.into_authentic(&self.keypair())
    }
}

pub trait PeerWrapperExt: PeerWrapper {
    fn generate_ed25519() -> PeerKp {
        PeerKp::generate_ed25519()
    }
    fn generate_noise_keys() -> NoiseKeys {
        NoiseKeys::new()
    }
}
