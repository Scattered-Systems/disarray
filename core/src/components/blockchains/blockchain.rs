/*
   Appellation: blockchain <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
           chain: HashMap<H256, Data>,
            map: HashMap<H256, MerkleMountainRange<Sha256, MemBackendVec<Hash>>>,
            tip: H256,
            depth: u128,
            num_pos: u128,
            num_pow: u128,
            epoch_size: u128,
            epoch_time: BlockTs,
            genesis_time: BlockTs,
            pub_len: u128,
            private_lead: u128,
*/
use crate::{blocks::{Block, generate_genesis_block}, validators::determine_block_validity, BlockTs};
use scsys::{crypto::hash::{H256, Hashable}, core::Timestamp};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockData {
    pub block: Block,
    pub height: u16,
}

impl BlockData {
    pub fn new(block: Block, height: u16) -> Self {
        Self { block, height }
    }

}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Epoch {
    size: u128,
    time: BlockTs
}

impl Epoch {
    pub fn new(size: u128, time: BlockTs) -> Self {
        Self { size, time }
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new(400, 120_000_000)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Position {
    pub depth: u128,
    pub pos: u128, // Number of proof-of-stake blocks
    pub pow: u128 // Number of proof-of-work blocks
}

impl Position {
    pub fn new(depth: u128, pos: u128, pow: u128) -> Self {
        Self { depth, pos, pow }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Blockchain {
    pub chain: HashMap<H256, BlockData>,
    pub epoch: Epoch,
    pub lead: u128,
    pub length: u128,
    pub position: Position,
    pub timestamp: i64,
    pub tip: H256
}

impl Blockchain {
    pub fn new(initial_time: BlockTs) -> Self {
        let genesis = generate_genesis_block(initial_time);
        log::info!("Created the genesis block with the timestamp: {}", genesis.header.timestamp);

        let data = BlockData::new(genesis.clone(), 0);
        let hash: H256 = genesis.clone().hash();

        let mut chain = HashMap::from([(hash, data)]);
        let mut map = HashMap::<String, String>::new();

        Self { 
            chain, 
            epoch: Epoch::default(), 
            lead: 0, 
            length: 0, 
            position: Position::default(), 
            timestamp: genesis.header.timestamp, 
            tip: hash
        }
    }
    pub fn add_block(&mut self, block: Block) -> &Self {

        self
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
