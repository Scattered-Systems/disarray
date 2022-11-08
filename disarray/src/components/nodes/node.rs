/*
   Appellation: node <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{peers::Peer, NetworkAddress};
use std::net::IpAddr;

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

impl std::convert::From<NetworkAddress> for Node {
    fn from(data: NetworkAddress) -> Self {
        Self::new(data, Default::default())
    }
}

impl std::convert::From<IpAddr> for Node {
    fn from(data: IpAddr) -> Self {
        Self::from(NetworkAddress::from(data))
    }
}

impl std::convert::From<&str> for Node {
    fn from(data: &str) -> Self {
        Self::from(data.parse::<IpAddr>().unwrap())
    }
}

impl std::convert::From<String> for Node {
    fn from(data: String) -> Self {
        Self::from(data.as_str())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::from(format!("0.0.0.0:{}", crate::MAINNET_PORT))
    }
}
