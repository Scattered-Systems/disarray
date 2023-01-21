/*
    Appellation: transport <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{peers::Peer, BoxedTransport, PeerKp};

use libp2p::{core::upgrade, mplex, noise, tcp, Transport};

pub fn tokio_transport(delay: bool, keypair: PeerKp) -> BoxedTransport {
    tcp::tokio::Transport::new(tcp::Config::default().nodelay(delay))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&keypair).expect(""))
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}

pub struct Conduit {
    pub transport: BoxedTransport,
}

impl Conduit {
    pub fn new(delay: bool, peer: Peer) -> Self {
        let transport = tokio_transport(delay, peer.keypair);
        Self {
            transport,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_transporter() {
        assert!(true)
    }
}
