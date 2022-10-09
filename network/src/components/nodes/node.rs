/*
   Appellation: node <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{peers::Peer, NetworkAddress};
use std::{net::IpAddr, str::FromStr};

/// Implement a standard peer on a p2p network
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
        let address = NetworkAddress::from(IpAddr::from_str("0.0.0.0:9090").ok().unwrap());
        Self::new(address, Vec::new())
    }
}
