/*
   Appellation: node <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::peers::Peer;

/// Outlines the standard Node structure to be used throughout the ecosystem
#[derive(Clone, Debug)]
pub struct Node {
    pub address: crate::NetworkAddress,
    pub peers: Vec<Peer>,
}

impl Node {
    fn constructor(
        address: crate::NetworkAddress,
        peers: Vec<Peer>,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self { address, peers })
    }
    pub fn new(address: crate::NetworkAddress, peers: Vec<Peer>) -> Self {
        match Self::constructor(address, peers) {
            Ok(v) => v,
            Err(e) => panic!("Node Error: {}", e),
        }
    }
    pub fn connect(address: crate::NetworkAddress) -> Self {
        Self::new(address, vec![Peer::new()])
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node(\naddress={:#?},\npeers={:#?}\n))",
            self.address, self.peers
        )
    }
}
