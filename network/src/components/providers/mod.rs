/*
   Appellation: providers <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{attr::*, provider::*, utils::*};

pub(crate) mod attr;
pub(crate) mod provider;

mod utils {
    use crate::{AuthNoiseKeys, BoxedTransport};
    use libp2p::{
        core::upgrade,
        mplex, noise,
        tcp::{GenTcpConfig, TokioTcpTransport},
        Transport,
    };

    pub fn create_tokio_transport(noise_keys: AuthNoiseKeys) -> BoxedTransport {
        TokioTcpTransport::new(GenTcpConfig::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}
