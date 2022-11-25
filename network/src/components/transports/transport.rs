/*
    Appellation: transport <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{BoxedTransport, AuthNoiseKeys};

use libp2p::{core::upgrade, mplex, noise, tcp, Transport};

pub struct Transporter(pub BoxedTransport);

impl Transporter {
    pub fn setup(dh_keys: AuthNoiseKeys, delay: bool) -> BoxedTransport {
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(delay))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(dh_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}

impl std::convert::From<AuthNoiseKeys> for Transporter {
    fn from(value: AuthNoiseKeys) -> Self {
        Self::from(Self::setup(value, false))
    }
}

impl std::convert::From<BoxedTransport> for Transporter {
    fn from(value: BoxedTransport) -> Self {
        Self(value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transporter() {

        assert!(true)
    }
}