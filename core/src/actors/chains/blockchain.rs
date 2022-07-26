/*
   Appellation: chain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::chains::Block;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Blockchain {
    pub address: std::net::SocketAddr,
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn constructor(address: std::net::SocketAddr, chain: Vec<Block>) -> Self {
        Self { address, chain }
    }

    pub fn new(address: std::net::SocketAddr) -> Self {
        Self::constructor(address.clone(), Vec::new())
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Blockchain(\naddress={:#?},\nchain={:#?}\n)",
            self.address, self.chain
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
