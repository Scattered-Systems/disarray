/*
   Appellation: node <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

use crate::{peers::StandardPeer, NetworkAddress};
use std::{net::IpAddr, str::FromStr};

/// Implement a standard peer on a p2p network
#[derive(Clone, Debug)]
pub struct StandardNode {
    pub address: NetworkAddress,
    pub peers: Vec<StandardPeer>,
}

impl StandardNode {
    pub fn new() -> Self {
        let address = NetworkAddress::from(IpAddr::from_str("0.0.0.0:9090").ok().unwrap());
        let peers = vec![StandardPeer::new()];
        Self { address, peers }
    }
}
