/*
   Appellation: chain
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

use crate::Block;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Blockchain<Addr = String> {
    pub address: Addr,
    pub chain: Vec<Block>,
}

impl<Addr> Blockchain<Addr> {
    pub fn constructor(address: Addr, chain: Vec<Block>) -> Self {
        Self { address, chain }
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
