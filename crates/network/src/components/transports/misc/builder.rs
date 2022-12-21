/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::transports::*;
use crate::{NoiseKeys, PeerKp};

#[derive(Clone)]
pub struct TransportBuilder {
    pub key: PeerKp,
    pub noise: NoiseKeys,
}

impl TransportBuilder {
    pub fn new(key: PeerKp, noise: NoiseKeys) -> Self {
        Self { key, noise }
    }
    fn dh_keys(&self) -> crate::AuthNoiseKeys {
        self.noise.clone().into_authentic(&self.key).ok().unwrap()
    }
    pub fn build(&self) -> Transporter {
        Transporter::new(false, self.dh_keys())
    }
}

// impl Transporter for TransportBuilder {
//     fn keypair(&self) -> &PeerKp {
//         &self.key
//     }
//     fn noise_keys(&self) -> &NoiseKeys {
//         &self.noise
//     }
// }

impl std::convert::From<&[u8]> for TransportBuilder {
    fn from(data: &[u8]) -> Self {
        Self::from(
            PeerKp::from_protobuf_encoding(data)
                .expect("Failed to load the transport from the given bytes"),
        )
    }
}

impl std::convert::From<PeerKp> for TransportBuilder {
    fn from(data: PeerKp) -> Self {
        Self::new(data, NoiseKeys::new())
    }
}
