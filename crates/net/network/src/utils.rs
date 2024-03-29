/*
   Appellation: utils <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{AuthNoiseKeys, BoxedTransport, NoiseKeys, NoiseResult, PeerKp};
use libp2p::{core::upgrade, mplex, noise, tcp, Transport};

pub fn authorize_peer(noise_keys: NoiseKeys, kp: PeerKp) -> NoiseResult {
    noise_keys.into_authentic(&kp)
}

pub fn generate_noise_keys() -> NoiseKeys {
    NoiseKeys::new()
}

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
