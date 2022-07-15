/*
   Appellation: node
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::Peer;

#[derive(Clone, Debug)]
pub struct Node<Addr = String> {
    pub address: Addr,
    pub peers: Vec<Peer>,
}

impl<Addr> Node<Addr> {
    pub fn constructor(address: Addr, peers: Vec<Peer>) -> Self {
        Self { address, peers }
    }
    pub fn from(address: Addr) -> Self {
        let mut peers = Vec::new();
        peers.push(Peer::new());
        Self::constructor(address, peers.clone())
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
