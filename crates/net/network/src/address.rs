/*
   Appellation: address <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkAddress(Multiaddr);

impl NetworkAddress {
    pub fn address(&self) -> Multiaddr {
        self.0.clone()
    }
}

impl Default for NetworkAddress {
    fn default() -> Self {
        Self::try_from("/ip4/127.0.0.1/tcp/9000".to_string()).unwrap()
    }
}

impl std::fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address())
    }
}

impl From<NetworkAddress> for libp2p::Multiaddr {
    fn from(value: NetworkAddress) -> libp2p::Multiaddr {
        value.address()
    }
}

impl From<libp2p::Multiaddr> for NetworkAddress {
    fn from(value: libp2p::Multiaddr) -> Self {
        Self(value)
    }
}

impl From<IpAddr> for NetworkAddress {
    fn from(value: IpAddr) -> Self {
        Self(Multiaddr::from(value))
    }
}

impl TryFrom<String> for NetworkAddress {
    type Error = libp2p::multiaddr::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let addr: libp2p::Multiaddr = value.parse()?;
        Ok(Self::from(addr))
    }
}

impl From<(&str, u16)> for NetworkAddress {
    fn from(data: (&str, u16)) -> Self {
        let _p = String::new();
        Self::try_from(format!("/ip4/{}/tcp/{}", data.0, data.1)).expect("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address_default() {
        let addr = NetworkAddress::default();
        let b: libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/9000".parse().unwrap();
        assert_eq!(addr.address(), b)
    }
}
