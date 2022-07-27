/*
    Appellation: config <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use libp2p::multihash::{Blake2b256, Hasher};
use libp2p::{mplex::MplexConfig, pnet::PreSharedKey};

/// Configuration type for [super::TransportBuilder]
#[derive(Clone, Debug)]
pub struct TransportConfig<'a> {
    /// Rendezvous string for configuring private network
    pub rendezvous_string: &'a [u8],
    /// Protocol name for configuring libp2p multiplexer
    pub mplex_protocol_name: &'static [u8],
}

impl<'a> TransportConfig<'a> {
    /// Gets [PreSharedKey] from the [TransportConfig] instance
    pub fn get_shared_key(&self) -> PreSharedKey {
        let mut hasher = Blake2b256::default();
        hasher.update(self.rendezvous_string);
        let hash = hasher.finalize();
        let mut psk_fixed: [u8; 32] = Default::default();
        psk_fixed.copy_from_slice(hash.as_ref());
        PreSharedKey::new(psk_fixed)
    }

    /// Gets [MplexConfig] from the [TransportConfig] instance
    pub fn get_mplex_config(&self) -> MplexConfig {
        let mut config = MplexConfig::new();
        config.set_protocol_name(self.mplex_protocol_name);
        config
    }
}

impl Default for TransportConfig<'_> {
    fn default() -> Self {
        Self {
            rendezvous_string: b"",
            mplex_protocol_name: b"/coda/mplex/1.0.0",
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
