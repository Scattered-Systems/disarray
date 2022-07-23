/*
   Appellation: peers <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use peer::*;
pub use utils::*;

mod peer;

mod utils {
    use crate::{AuthNoiseKeys, NoiseKeys, PeerKP};

    pub fn create_auth_noise_keys(key: &PeerKP) -> AuthNoiseKeys {
        match NoiseKeys::new().into_authentic(&key) {
            Ok(v) => v,
            Err(e) => {
                panic!("Signing Error: {}", e)
            }
        }
    }
}
