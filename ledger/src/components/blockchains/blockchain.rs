/*
    Appellation: blockchain <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{BlockData, ChainWrapper, ChainWrapperExt, CoreChainSpec, Epoch, Position};
use crate::blocks::{generate_genesis_block, Block, BlockHeader, BlockHeaderSpec, CoreBlockSpec};
use scsys::prelude::{
    rand::{self, Rng},
    Hashable, Timestamp, H160, H256,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: HashMap<H256, BlockData>,
    pub epoch: Epoch,
    pub lead: u128,
    pub length: u128,
    pub map: HashMap<H256, HashMap<H256, H160>>,
    pub position: Position,
    pub timestamp: i64, // The time of creation (genesis_timestamp)
    pub tip: H256,
}

impl Blockchain {
    pub fn new(timestamp: i64) -> Self {
        let genesis = generate_genesis_block(timestamp);
        log::info!(
            "Created the genesis block with the timestamp: {}",
            genesis.header.timestamp
        );

        let data = BlockData::new(genesis.clone(), 0);
        let hash: H256 = genesis.hash();

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
    pub fn insert_selfish_pos(&mut self, block: &Block) -> bool {
        // Insert a block into blockchain as a selfish miner
        if self.is_block(&block.hash()) {
            false
        } else {
            let header: BlockHeader = block.header().clone();
            let parenthash: H256 = header.parent();
            let parentdata: BlockData = match self.chain.get(&parenthash) {
                Some(data) => data.clone(),
                None => return false,
            };
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            self.position.pos += 1;
            if newheight > self.position.depth && block.selfish_block {
                self.lead += 1;
                self.position.depth = newheight;
                self.tip = newhash;
                return true;
            } else if !block.selfish_block && newheight > self.length {
                if self.lead > 0 {
                    self.lead -= 1;
                    self.length += 1;
                    return false;
                } else {
                    self.position.depth = newheight;
                    self.tip = newhash;
                    self.length = newheight;
                    return true;
                }
            }
            false
        }
    }
    pub fn insert_unselfish_pos(&mut self, block: &Block) -> bool {
        if self.chain.contains_key(&block.hash()) {
            false
        } else {
            let pdata: BlockData = match self.find_one_payload(&block.header.parent()) {
                Some(v) => v,
                None => return false,
            };
            let height = pdata.height + 1;
            let data = BlockData::new(block.clone(), height);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, data);
            // self.map.insert(newhash, new_mmr);
            self.position.pos += 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if height > self.position.depth
                || (height == self.position.depth && block.selfish_block && p < 1.0)
            {
                self.position.depth = height;
                self.tip = newhash;
                return true;
            }
            false
        }
    }

    /// General access for inserting new blocks created from staking
    pub fn insert_pos(&mut self, block: &Block, selfish: bool) -> bool {
        if !selfish {
            self.insert_unselfish_pos(block)
        } else {
            self.insert_selfish_pos(block)
        }
    }
    /// Insert a PoW block into blockchain
    pub fn insert_pow(&mut self, block: &Block) -> bool {
        //unimplemented!()
        if self.is_block(&block.hash()) {
            false
        } else {
            let prev: BlockData = match self.find_one_payload(&block.header().parent()) {
                None => return false,
                Some(v) => v,
            };
            let data = BlockData::new(block.clone(), prev.height + 1);
            let hash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(hash, data);
            // self.map.insert(newhash, new_mmr);
            self.position.pow += 1;

            true
        }
    }
}

impl CoreChainSpec for Blockchain {
    fn chain(&self) -> &HashMap<H256, BlockData> {
        &self.chain
    }

    fn epoch(&self) -> &Epoch {
        &self.epoch
    }

    fn tip(&self) -> H256 {
        self.tip
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

impl ChainWrapper for Blockchain {}

impl ChainWrapperExt for Blockchain {
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
        let hash: H256 = genesis.hash();

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

impl Default for Blockchain {
    fn default() -> Self {
        Self::new(Timestamp::timestamp())
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.chain).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blockchain_genesis() {
        let a = Blockchain::default();

        assert!(a.contains_hash(a.chain.keys().last().unwrap()));
    }
}
