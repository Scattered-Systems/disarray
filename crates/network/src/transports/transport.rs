/*
    Appellation: transport <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::behaviours::mainnet::MainnetBehaviour;
use crate::{AuthNoiseKeys, BoxedTransport, PeerId};

use libp2p::{core::upgrade, mplex, noise, swarm::Swarm, tcp, Transport};

pub fn tokio_transport(delay: bool, dh_keys: AuthNoiseKeys) -> BoxedTransport {
    libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(delay))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseConfig::xx(dh_keys).into_authenticated()
        )
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}

pub struct Conduit {
    pub swarm: Arc<Swarm<MainnetBehaviour>>,
    pub transport: BoxedTransport
}

impl Conduit {
    pub fn new(behaviour: MainnetBehaviour, pid: PeerId, transport: BoxedTransport) -> Self {
        let swarm = Swarm::<MainnetBehaviour>::with_tokio_executor(transport, behaviour, pid);
        Self { swarm: Arc::new(swarm), transport }
    }
}

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
    pub fn swarm(&self, behaviour: MainnetBehaviour, pid: PeerId) -> Swarm<MainnetBehaviour> {
        Swarm::with_tokio_executor(self.transport(), behaviour, pid)
    }
    pub fn transport(&self) -> BoxedTransport {
        tokio_transport(self.delay.clone(), self.dh_keys.clone())
    }
}

impl std::convert::From<AuthNoiseKeys> for Transporter {
    fn from(value: AuthNoiseKeys) -> Self {
        Self::new(false, value)
    }
}

impl From<Transporter> for BoxedTransport {
    fn from(t: Transporter) -> BoxedTransport {
        self.transport()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_transporter() {
        assert!(true)
    }
}
