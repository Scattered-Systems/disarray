/*
   Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{peers::Peer, NetworkAddress};

/// This structure implements the node framework expected for network participants
#[derive(Clone, Debug)]
pub struct Node {
    pub address: NetworkAddress,
    pub peers: Vec<Peer>,
}

impl Node {
    pub fn new(address: NetworkAddress, peers: Vec<Peer>) -> Self {
        Self { address, peers }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(Default::default(), vec![])
    }
}
