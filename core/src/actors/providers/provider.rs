/*
   Appellation: provider
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::Node;

#[derive(Clone, Debug)]
pub struct Provider<Addr = String> {
    pub address: Addr,
    pub nodes: Vec<Node>,
}

impl<Addr> Provider<Addr> {
    pub fn constructor(address: Addr, nodes: Vec<Node>) -> Self {
        Self { address, nodes }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Provider(\naddress={:#?},\nnodes={:#?}\n)",
            self.address, self.nodes
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
