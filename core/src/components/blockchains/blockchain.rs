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
use crate::{
    blocks::{generate_genesis_block, Block, BlockHeader},
    BlockTs,
};
use scsys::{
    crypto::hash::{hash_divide_by, Hashable, H160, H256},
    prelude::rand::{self, Rng},
};
use std::collections::{HashMap, HashSet};

pub trait Blocker {
    fn chain(&self) -> &HashMap<H256, BlockData>;
    fn epoch(&self) -> &Epoch;
    fn tip(&self) -> H256;
    fn lead(&self) -> u128;
    fn length(&self) -> u128;
    fn map(&self) -> &HashMap<H256, HashMap<H256, H160>>;
    fn position(&self) -> &Position;
    fn timestamp(&self) -> i64; // genesis timestamp
}

pub trait BlockerExt: Blocker {
    fn contains_hash(&self, hash: &H256) -> bool {
        self.chain().contains_key(hash)
    }
    fn genesis(timestamp: i64) -> Block {
        let block = generate_genesis_block(timestamp);
        log::info!(
            "Created the genesis block with the timestamp: {}",
            &block.header.timestamp
        );
        block
    }
    fn get_all_blocks_from_longest(&self) -> Vec<H256> {
        let mut blocks: Vec<H256> = vec![];
        let mut current_hash = self.tip();
        //let mut parent_hash;
        let mut pdata: BlockData;

        loop {
            match self.chain().get(&current_hash) {
                None => break,
                Some(b) => pdata = b.clone(),
            }
            blocks.push(current_hash);
            current_hash = pdata.block.header.parent;
        }
        log::debug!("finish {:?}!", blocks);

        blocks.reverse();
        blocks
    }
}

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
        //unimplemented!()
        if !selfish {
            if self.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData;
            match self.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => return false,
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            self.position.pos = self.position.pos + 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if newheight > self.position.depth
                || (newheight == self.position.depth && block.selfish_block == true && p < 1.0)
            {
                self.position.depth = newheight;
                self.tip = newhash;
                return true;
            }
            return false;
        } else {
            // Insert a block into blockchain as a selfish miner
            if self.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData;
            match self.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => return false,
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            self.position.pos = self.position.pos + 1;
            if newheight > self.position.depth && block.selfish_block == true {
                self.lead = self.lead + 1;
                self.position.depth = newheight;
                self.tip = newhash;
                return true;
            } else if block.selfish_block == false && newheight > self.length {
                if self.lead > 0 {
                    self.lead = self.lead - 1;
                    self.length = self.length + 1;
                    return false;
                } else {
                    self.position.depth = newheight;
                    self.tip = newhash;
                    self.length = newheight;
                    return true;
                }
            }
            return false;
        }
    }
    /// Insert a PoW block into blockchain
    pub fn insert_pow(&mut self, block: &Block) -> bool {
        //unimplemented!()
        if self.chain.contains_key(&block.hash()) {
            return false;
        }
        let header: BlockHeader = block.header.clone();
        let parenthash: H256 = header.parent;
        let parentdata: BlockData;
        match self.chain.get(&parenthash) {
            Some(data) => parentdata = data.clone(),
            None => return false,
        }
        let parentheight = parentdata.height;
        let newheight = parentheight + 1;
        let newdata = BlockData::new(block.clone(), newheight);
        let newhash = block.hash();
        // let mut new_mmr = self.get_mmr(&parenthash);
        // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
        self.chain.insert(newhash, newdata);
        // self.map.insert(newhash, new_mmr);
        self.position.pow = self.position.pow + 1;

        return true;
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
    fn genesis(timestamp: i64) -> Self
    where
        Self: Sized,
    {
        let genesis = generate_genesis_block(timestamp);
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
