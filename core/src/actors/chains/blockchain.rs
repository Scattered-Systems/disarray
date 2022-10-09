/*
   Appellation: chain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blocks::Block, determine_block_validity};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
        let block = Block::new(0u64, "genesis".to_string(), Vec::new());
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
