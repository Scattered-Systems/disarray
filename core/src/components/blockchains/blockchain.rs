/*
   Appellation: blockchain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blocks::Block, validators::determine_block_validity};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockData {
    pub block: Block,
    pub height: u16,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Blockchain {
    pub blockchain: Vec<BlockData>
}

impl Blockchain {
    pub fn new() -> Self {
        let blockchain = Vec::new();
        Self { blockchain }
    }
    pub fn add_block(&mut self, block: Block) {
        let pblock = self.blockchain.last().unwrap().clone();
        if determine_block_validity(&block, &pblock.block) == true {
            self.blockchain.push(BlockData { block, height: 0 })
        } else {
            panic!("Invalid Block")
        }
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
