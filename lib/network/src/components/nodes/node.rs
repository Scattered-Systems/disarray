/*
   Appellation: node <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{peers::Peer, NetworkAddress};
use std::{net::IpAddr, str::FromStr};

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
        Self::from(
            NetworkAddress::from_str(data)
                .expect("Failed to parse the input for a valid ip address"),
        )
    }
}

impl std::convert::From<String> for Node {
    fn from(data: String) -> Self {
        Self::from(data.as_str())
    }
}

impl std::convert::From<(&str, u16)> for Node {
    fn from(data: (&str, u16)) -> Self {
        let _p = String::new();
        Self::from(format!("/ip4/{}/tcp/{}", data.0, data.1))
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::from(("127.0.0.1", crate::MAINNET_PORT))
    }
}
