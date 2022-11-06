/*
    Appellation: interface <transports>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{BoxedTransport, NoiseKeys, NoiseResult, PeerId, PeerKp};
use libp2p::{core::upgrade, mplex, noise, tcp, Transport};

pub trait Transporter {
    fn keypair(&self) -> &PeerKp;
    fn noise_keys(&self) -> &NoiseKeys;
    fn peer_id(&self) -> PeerId {
        PeerId::from(self.keypair().public())
    }
    fn version(&self) -> upgrade::Version {
        upgrade::Version::V1
    }
}

pub trait TransportWrapper: Transporter {
    fn auth(&self) -> NoiseResult<noise::NoiseAuthenticated<noise::XX, noise::X25519Spec, ()>> {
        noise::NoiseAuthenticated::xx(self.keypair())
    }
}

pub trait TransportWrapperExt: TransportWrapper {
    fn is_authenticated(&self) -> bool {
        self.auth().is_ok()
    }
    fn quickstart_tcp(&self) -> BoxedTransport {
        tcp::TokioTcpTransport::new(tcp::GenTcpConfig::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                self.auth()
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}
