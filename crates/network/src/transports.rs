/*
    Appellation: transport <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{peers::Peer, BoxedTransport, PeerKp, AuthNoiseKeys};

use libp2p::{core::upgrade, mplex, noise, tcp, Transport};

pub fn tokio_transport_noise(delay: bool, dh_keys: AuthNoiseKeys) -> BoxedTransport {
    tcp::tokio::Transport::new(tcp::Config::default().nodelay(delay))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(dh_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}

pub fn tokio_transport(delay: bool, keypair: PeerKp) -> BoxedTransport {
    tcp::tokio::Transport::new(tcp::Config::default().nodelay(delay))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&keypair).expect(""))
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}

pub struct TransportBuilder {
    pub delay: bool,
    pub peer: Peer
}

impl TransportBuilder {
    pub fn new(delay: bool, peer: Peer) -> Self {
        Self { delay, peer }
    }
    pub fn transport(&self) -> BoxedTransport {
        tokio_transport(self.delay, self.peer.keypair.clone())
    }
}

pub struct Conduit {
    pub transport: BoxedTransport,
}

impl Conduit {
    pub fn new(delay: bool, peer: Peer) -> Self {
        let transport = tokio_transport(delay, peer.keypair);
        Self { transport }
    }
}
impl From<TransportBuilder> for BoxedTransport {
    fn from(value: TransportBuilder) -> Self {
        tokio_transport(value.delay, value.peer.keypair.clone())
    }
}

impl From<Conduit> for BoxedTransport {
    fn from(value: Conduit) -> Self {
        value.transport
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_transporter() {
        assert!(true)
    }
}
