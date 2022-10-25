/*
   Appellation: blockchain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blocks::Block, validators::determine_block_validity};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub port: u16
}

impl Blockchain {
    pub fn new(port: u16) -> Self {
        let chain = Vec::new();
        Self { chain, port }
    }
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.port))
    }
    pub fn genesis(&mut self) -> Self {
        let block = Block::new(0u64, "genesis".to_string(), Vec::new());
        self.chain.push(block);
        self.clone()
    }
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
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
        Self::new( 9090)
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Blockchain(\naddress={:#?},\nchain={:#?}\n)",
            self.address(), self.chain
        )
    }
}
