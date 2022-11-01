/*
    Appellation: blockchain <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{
    chain_data::BlockData,
    pieces::{Epoch, Position},
};
use crate::blocks::Block;
use scsys::prelude::{Hashable, H160, H256};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: HashMap<H256, BlockData>,
    pub epoch: Epoch,
    pub lead: u128,
    pub length: u128,
    pub map: HashMap<H256, HashMap<H256, H160>>,
    pub position: Position,
    pub timestamp: i64, // genesis timestamp
    pub tip: H256,
}

impl Blockchain {
    // pub fn get_mmr(&self, hash: &H256) -> MerkleMountainRange<Sha256, Vec<Hash>> {
    //     let mmr_ref = self.map.get(hash).unwrap();
    //     let leaf_hashes = mmr_ref
    //         .get_leaf_hashes(0, mmr_ref.get_leaf_count().unwrap() + 1)
    //         .unwrap()
    //         .clone();
    //     let mut mmr_ret = MerkleMountainRange::<Sha256, Vec<Hash>>::new(Vec::new());
    //     mmr_ret.assign(leaf_hashes).unwrap();
    //     mmr_ret
    // }

    /// Insert a PoS block into blockchain
    pub fn insert_pos(&mut self, block: &Block, selfish: bool) -> bool {
        super::insert_pos(self, block, selfish)
    }
    /// Insert a PoW block into blockchain
    pub fn insert_pow(&mut self, block: &Block) -> bool {
        super::insert_pow(self, block)
    }
}

impl super::ChainWrapper for Blockchain {
    fn chain(&self) -> &HashMap<H256, BlockData> {
        &self.chain
    }

    fn epoch(&self) -> &Epoch {
        &self.epoch
    }

    fn tip(&self) -> H256 {
        self.tip.clone()
    }

    fn map(&self) -> &HashMap<H256, HashMap<H256, H160>> {
        &self.map
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }

    fn lead(&self) -> u128 {
        self.lead
    }

    fn length(&self) -> u128 {
        self.length
    }
}

impl super::ChainWrapperExt for Blockchain {
    fn genesis(blockgen: fn(i64) -> Block, timestamp: i64) -> Self
    where
        Self: Sized,
    {
        let genesis = blockgen(timestamp);
        log::info!(
            "Created the genesis block with the timestamp: {}",
            genesis.header.timestamp
        );

        let data = BlockData::new(genesis.clone(), 0);
        let hash: H256 = genesis.clone().hash();

        // let mmr: MerkleMountainRange<Sha256, Vec<Vec<u8>>> = MerkleMountainRange::new(Vec::new());
        let map = HashMap::new();

        Self {
            chain: HashMap::from([(hash, data)]),
            epoch: Epoch::default(),
            lead: 0,
            length: 0,
            map,
            position: Position::default(),
            timestamp: genesis.header.timestamp,
            tip: hash,
        }
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
