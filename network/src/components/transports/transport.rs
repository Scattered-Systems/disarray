/*
    Appellation: transport <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{TransportWrapper, TransportWrapperExt, Transporter};
use crate::{NoiseKeys, PeerKp};

#[derive(Clone)]
pub struct Transport {
    pub key: PeerKp,
    pub noise: NoiseKeys,
}

impl Transport {
    pub fn new(key: PeerKp, noise: NoiseKeys) -> Self {
        Self { key, noise }
    }
}

impl Transporter for Transport {
    fn keypair(&self) -> &PeerKp {
        &self.key
    }
    fn noise_keys(&self) -> &NoiseKeys {
        &self.noise
    }
}

impl TransportWrapper for Transport {}

impl TransportWrapperExt for Transport {}

impl std::convert::From<&[u8]> for Transport {
    fn from(data: &[u8]) -> Self {
        Self::from(
            PeerKp::from_protobuf_encoding(data)
                .expect("Failed to load the transport from the given bytes"),
        )
    }
}

impl std::convert::From<PeerKp> for Transport {
    fn from(data: PeerKp) -> Self {
        Self::new(data, NoiseKeys::new())
    }
}
