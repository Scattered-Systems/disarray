/*
    Appellation: transport <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::behaviours::mainnet::MainnetBehaviour;
use crate::{AuthNoiseKeys, BoxedTransport, PeerId};

use libp2p::{core::upgrade, mplex, noise, swarm::Swarm, tcp, Transport};

/// A functional wrapper for libp2p's Transport mechanism
#[derive(Clone)]
pub struct Transporter {
    delay: bool,
    dh_keys: AuthNoiseKeys,
}

impl Transporter {
    pub fn new(delay: bool, dh_keys: AuthNoiseKeys) -> Self {
        Self { delay, dh_keys }
    }
    pub fn setup(&self) -> BoxedTransport {
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(self.delay))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(self.dh_keys.clone()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
    pub fn swarm(&self, behaviour: MainnetBehaviour, pid: PeerId) -> Swarm<MainnetBehaviour> {
        Swarm::with_tokio_executor(self.transport(), behaviour, pid)
    }
    pub fn transport(&self) -> BoxedTransport {
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(self.delay))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(self.dh_keys.clone()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}

impl std::convert::From<AuthNoiseKeys> for Transporter {
    fn from(value: AuthNoiseKeys) -> Self {
        Self::new(false, value)
    }
}

// impl std::convert::From<BoxedTransport> for Transporter {
//     fn from(value: BoxedTransport) -> Self {
//         Self(value)
//     }
// }

#[cfg(test)]
mod tests {

    #[test]
    fn test_transporter() {
        assert!(true)
    }
}
