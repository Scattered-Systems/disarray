/*
   Appellation: provider
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use libp2p::{Transport, PeerId, core::{transport::Boxed, muxing::StreamMuxerBox}};

pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

#[derive(Debug)]
pub struct Provider<Addr = String> {
    pub address: Addr,
    pub clients: Vec<BoxedTransport>,
}

impl<Addr> Provider<Addr> {
    pub fn constructor(address: Addr, clients: Vec<BoxedTransport>) -> Self {
        Self { address, clients }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Provider(\naddress={:#?})",
            self.address
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
