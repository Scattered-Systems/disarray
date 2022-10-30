/*
    Appellation: blockchain <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{pieces::{Epoch, Position}, chain_data::BlockData};
use crate::{
    blocks::{generate_genesis_block, Block, BlockHeader},
    BlockTs,
};
use scsys::{
    crypto::hash::{hash_divide_by, Hashable, H256},
    prelude::rand::{self, Rng},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Blockchain {
    pub chain: HashMap<H256, BlockData>,
    pub epoch: Epoch,
    pub lead: u128,
    pub length: u128,
    pub position: Position,
    pub timestamp: i64, // genesis timestamp
    pub tip: H256,
}

impl Blockchain {
    pub fn new(initial_time: BlockTs) -> Self {
        let genesis = generate_genesis_block(initial_time);
        log::info!(
            "Created the genesis block with the timestamp: {}",
            genesis.header.timestamp
        );

        let data = BlockData::new(genesis.clone(), 0);
        let hash: H256 = genesis.clone().hash();
        
        let mut map = HashMap::<String, String>::new();

        Self {
            chain: HashMap::from([(hash, data)]),
            epoch: Epoch::default(),
            lead: 0,
            length: 0,
            position: Position::default(),
            timestamp: genesis.header.timestamp,
            tip: hash,
        }
    }
    /// Get the last block's hash of the longest chain
    //#[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        //unimplemented!()
        let mut all_block: Vec<H256> = vec![];
        let mut current_hash = self.tip;
        //let mut parent_hash;
        let mut parentdata: BlockData;

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => parentdata = data.clone(),
            }
            all_block.push(current_hash);
            current_hash = parentdata.block.header.parent;
        }
        log::debug!("finish {:?}!", all_block);

        all_block.reverse();
        all_block
    }
    pub fn contains_hash(&self, hash: &H256) -> bool {
        self.chain.contains_key(hash)
    }
    pub fn epoch(&self, current_ts: BlockTs) -> BlockTs {
        let epoch_size = self.epoch.size;
        //let depth = self.depth;
        let epoch_time = self.epoch.time;
        //let tip = self.tip;
        //let tip_time = self.chain.get(&tip).unwrap().blk.header.timestamp;
        let genesis_time = self.timestamp;
        //let tip_epoch = (tip_time - genesis_time)/epoch_time;
        let current_epoch = (current_ts - genesis_time) / epoch_time;
        current_epoch
    }
    pub fn find_one_block(&self, hash: &H256) -> Option<Block> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.block.clone()),
        }
    }
    pub fn find_one_depth(&self, hash: &H256) -> Option<u128> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.height),
        }
    }
    pub fn find_one_header(&self, hash: &H256) -> Option<BlockHeader> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.block.header.clone()),
        }
    }
    pub fn find_one_height(&self, height: u128) -> H256 {
        let mut current_hash = self.tip;
        //let parent_hash: H256 = hash.clone();
        let mut childdata: BlockData;

        loop {
            childdata = self.chain.get(&current_hash).unwrap().clone();
            if childdata.height == height {
                return childdata.block.hash().clone();
            }
            current_hash = childdata.block.header.parent.clone();
        }
    }
    pub fn get_chain_quality(&self) -> f32 {
        //unimplemented!()
        // let mut all_block : Vec<H256> = vec![];
        let mut current_hash = self.tip;
        let mut parentdata: BlockData;
        let mut count = 0;
        let mut count_selfish = 0;
        let mut all_pow_hash: Vec<H256> = Vec::new();

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => parentdata = data.clone(),
            }
            //all_block.push(current_hash);
            let pow_hashes = parentdata.block.content.reference.clone();
            for pow_hash in pow_hashes {
                if !all_pow_hash.contains(&pow_hash) {
                    all_pow_hash.push(pow_hash);
                    count = count + 1;
                    let pow_block = self.find_one_block(&pow_hash).unwrap().clone();
                    if pow_block.selfish_block == true {
                        count_selfish = count_selfish + 1;
                    }
                }
            }

            current_hash = parentdata.block.header.parent;
        }
        let chain_quality: f32 = 1.0 - (count_selfish as f32) / (count as f32);
        chain_quality
    }
    pub fn get_current_position(&self) -> &Position {
        &self.position
    }
    pub fn get_depth(&self) -> u128 {
        self.position.depth
    }
    pub fn get_lead(&self) -> u128 {
        self.lead
    }
    pub fn get_length(&self) -> u128 {
        self.length
    }
    pub fn get_longest_chain(&self) -> Vec<Block> {
        //unimplemented!()
        let mut all_block: Vec<H256> = vec![];
        let mut current_hash = self.tip;
        //let mut parent_hash;
        let mut pdata: BlockData;

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => pdata = data.clone(),
            }
            all_block.push(current_hash);
            current_hash = pdata.block.header.parent;
        }
        all_block.reverse();
        log::debug!("finish {:?}!", all_block);

        let mut chain: Vec<Block> = vec![];
        for hash in all_block {
            chain.push(self.find_one_block(&hash).unwrap().clone());
        }
        chain
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
    pub fn get_num_pos(&self) -> u128 {
        self.position.pos
    }
    pub fn get_num_pow(&self) -> u128 {
        self.position.pow
    }
    pub fn get_pos_difficulty(&self) -> H256 {
        // should be parent, but it's okay since all pos are the same
        self.chain
            .get(&self.tip)
            .unwrap()
            .block
            .header
            .pos_difficulty
    }
    pub fn get_pow_difficulty(&self, current_ts: BlockTs, parent: H256) -> H256 {
        let epoch_size = self.epoch.size;
        let depth = self.position.depth;
        let epoch_time = self.epoch.time;
        let parent_time = self.chain.get(&parent).unwrap().block.header.timestamp;
        let genesis_time = self.timestamp;
        let parent_epoch = (parent_time - genesis_time) / epoch_time;
        let curent_epoch = (current_ts - genesis_time) / epoch_time;
        if curent_epoch > parent_epoch && depth > 1 {
            let old_diff: H256 = self.chain.get(&parent).unwrap().block.header.pow_difficulty;
            let mut hash = parent.clone();
            let mut all_hashs = Vec::new();
            while true {
                let blk = self.chain.get(&hash).unwrap().block.clone();
                let pow_blks = blk.content.reference.clone();
                for pow_blk in pow_blks {
                    if !all_hashs.contains(&pow_blk) {
                        all_hashs.push(pow_blk);
                    }
                }
                hash = self.chain.get(&hash).unwrap().block.header.parent;
                let blk_time = self.chain.get(&hash).unwrap().block.header.timestamp;
                let blk_epoch = (blk_time - genesis_time) / epoch_time;
                if blk_epoch < parent_epoch || blk_time == self.timestamp {
                    break;
                }
            }
            let num_blk = all_hashs.len();
            //let start_time: u128 = self.chain.get(&hash).unwrap().blk.header.timestamp;
            let ratio = (num_blk as f64) / (epoch_size as f64);
            //println!("Ratio: {}", ratio);
            // if ratio > 4.0 {
            // 	ratio = 4.0;
            // } else if ratio < 0.25 {
            // 	ratio = 0.25;
            // }
            let new_diff: H256 = hash_divide_by(&old_diff, ratio);
            log::debug!(
                "Mining difficulty changes from {} to {}",
                old_diff,
                new_diff
            );
            new_diff
        } else {
            self.chain.get(&parent).unwrap().block.header.pow_difficulty
        }
    }
    pub fn get_size(&self) -> usize {
        self.chain.len()
    }
    pub fn is_new_epoch_and_count_blocks(
        &self,
        current_ts: BlockTs,
    ) -> Option<HashMap<Vec<u8>, HashSet<H256>>> {
        let epoch_size = self.epoch.size;
        let depth = self.position.depth;
        let epoch_time = self.epoch.time;
        let tip = self.tip;
        let tip_time = self.chain.get(&tip).unwrap().block.header.timestamp;
        let genesis_time = self.timestamp;
        let tip_epoch = (tip_time - genesis_time) / epoch_time;
        let curent_epoch = (current_ts - genesis_time) / epoch_time;
        // if it is new epoch, count last epoch's blocks
        let mut tip_iter = tip;
        if curent_epoch > tip_epoch {
            let mut cnt: HashMap<Vec<u8>, HashSet<H256>> = HashMap::new();
            loop {
                let b = &self.chain.get(&tip_iter).unwrap().block;
                if b.header.timestamp - genesis_time == 0 {
                    break;
                }
                let this_epoch = (b.header.timestamp - genesis_time) / epoch_time;
                if this_epoch != tip_epoch {
                    break;
                }
                for h in b.content.reference.iter() {
                    let ref_b = &self
                        .chain
                        .get(h)
                        .expect("error, transaction ref is not in blockchain!!!")
                        .block;
                    let miner = ref_b.header.vrf_pub_key.clone();
                    if let Some(m) = cnt.get_mut(&miner) {
                        m.insert(h.clone());
                    } else {
                        let mut m = HashSet::new();
                        m.insert(h.clone());
                        cnt.insert(miner, m);
                    }
                }
                tip_iter = b.header.parent;
            }
            return Some(cnt);
        }
        // is not new epoch, return none
        None
    }
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
            /// Insert a block into blockchain as a selfish miner
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
    pub fn print_longest_chain(&self) {
        log::info!("************* Print Longest Chain *************");
        log::info!("{:?}", self.all_blocks_in_longest_chain());
        log::info!("***********************************************");
    }
    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        //unimplemented!()
        self.tip
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
