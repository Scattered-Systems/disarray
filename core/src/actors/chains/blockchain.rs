/*
   Appellation: chain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blocks::Block, determine_block_validity};
use std::net::SocketAddr;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Blockchain {
    pub address: std::net::SocketAddr,
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            chain: Vec::new(),
        }
    }
    pub fn genesis(&mut self) -> Self {
        let block = Block::new(0u64, "genesis".to_string(), Vec::<String>::new());
        self.chain.push(block);
        self.clone()
    }
    pub fn set_address(&mut self, address: SocketAddr) {
        self.address = address;
    }
    pub fn add_block(&mut self, block: Block) {
        if determine_block_validity(&block, &self.chain.last().unwrap().clone()) == true {
            self.chain.push(block)
        } else {
            panic!("Invalid Block")
        }
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new(SocketAddr::from(([0, 0, 0, 0], 9090)))
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
    use super::Blockchain;
    use crate::blocks::Block;

    #[test]
    fn test_blockchain_default() {
        let a = Blockchain::default();
        let b = Blockchain::new(std::net::SocketAddr::from(([0, 0, 0, 0], 9090)));
        assert_eq!(a, b)
    }

    #[test]
    fn test_blockchain_update() {
        let mut blockchain = Blockchain::default().genesis();
        blockchain.add_block(Block::<String>::new(
            1u64,
            blockchain.chain.last().unwrap().hash.clone(),
            Vec::new(),
        ));
        assert!(crate::determine_chain_validity(&blockchain.chain))
    }
}
