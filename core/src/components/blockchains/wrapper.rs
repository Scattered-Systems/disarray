/*
    Appellation: wrapper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{BlockData, Epoch, Position};
use crate::{
    blocks::{generate_genesis_block, Block, BlockHeader},
    BlockTs,
};
use scsys::crypto::hash::{Hashable, H160, H256};
use std::collections::{HashMap, HashSet};

pub trait GenesisBlock {
    fn genesis(timestamp: i64) -> Block {
        generate_genesis_block(timestamp)
    }
}

pub trait ChainWrapper {
    fn chain(&self) -> &HashMap<H256, BlockData>;
    fn chain_fetch<T>(&self, data: &H256, catalyst: fn(&BlockData) -> T) -> Option<T> {
        self.chain().get(data).map(catalyst)
    }
    fn chain_size(&self) -> usize {
        self.chain().len()
    }
    fn depth(&self) -> u128 {
        self.position().depth
    }
    fn epoch(&self) -> &Epoch;
    fn epoch_current(&self, current_ts: BlockTs) -> BlockTs {
        let epoch_time = self.epoch().time;
        let genesis_time = self.timestamp();
        (current_ts - genesis_time) / epoch_time
    }
    fn tip(&self) -> H256;
    fn lead(&self) -> u128;
    fn length(&self) -> u128;
    fn map(&self) -> &HashMap<H256, HashMap<H256, H160>>;
    fn position(&self) -> &Position;

    fn position_pos(&self) -> u128 {
        self.position().pos
    }
    fn position_pow(&self) -> u128 {
        self.position().pow
    }
    fn timestamp(&self) -> i64; // genesis timestamp
}

pub trait ChainWrapperExt: ChainWrapper {
    fn contains_hash(&self, hash: &H256) -> bool {
        self.chain().contains_key(hash)
    }
    ///
    fn find_one_block(&self, hash: &H256) -> Option<Block> {
        let catalyst = |v: &BlockData| v.block.clone();
        self.chain_fetch(hash, catalyst)
    }
    ///
    fn find_one_depth(&self, hash: &H256) -> Option<u128> {
        let catalyst = |v: &BlockData| v.height;
        self.chain_fetch(hash, catalyst)
    }
    ///
    fn find_one_header(&self, hash: &H256) -> Option<BlockHeader> {
        let catalyst = |v: &BlockData| v.block.header.clone();
        self.chain_fetch(hash, catalyst)
    }
    ///
    fn find_one_height(&self, height: u128) -> H256 {
        let mut curhash = self.tip();
        //let parent_hash: H256 = hash.clone();
        let mut child: BlockData;

        loop {
            child = self.chain().get(&curhash).unwrap().clone();
            if child.height == height {
                return child.block.hash();
            }
            curhash = child.block.header.parent;
        }
    }
    /// Create a new blockchain
    fn genesis(blockgen: fn(i64) -> Block, timestamp: i64) -> Self
    where
        Self: Sized;
    /// TODO: Finalize the chain quality
    fn get_chain_quality(&self) -> f32 {
        //unimplemented!()
        // let mut all_block : Vec<H256> = vec![];
        let mut current_hash = self.tip();
        let mut parentdata: BlockData;
        let mut count = 0;
        let mut count_selfish = 0;
        let mut all_pow_hash: Vec<H256> = Vec::new();

        loop {
            match self.chain().get(&current_hash) {
                Some(data) => parentdata = data.clone(),
                None => break,
            }
            //all_block.push(current_hash);
            let pow_hashes = parentdata.block.content.reference.clone();
            for pow_hash in pow_hashes {
                if !all_pow_hash.contains(&pow_hash) {
                    all_pow_hash.push(pow_hash);
                    count += 1;
                    let pow_block = self.find_one_block(&pow_hash).unwrap().clone();
                    if pow_block.selfish_block {
                        count_selfish += 1;
                    }
                }
            }

            current_hash = parentdata.block.header.parent;
        }
        let chain_quality: f32 = 1.0 - (count_selfish as f32) / (count as f32);
        chain_quality
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
    fn get_longest_chain(&self) -> Vec<Block> {
        //unimplemented!()
        let mut all_block: Vec<H256> = vec![];
        let mut current_hash = self.tip();
        //let mut parent_hash;
        let mut pdata: BlockData;

        loop {
            match self.chain().get(&current_hash) {
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
    // TODO: Make parent, however, currently functional since all pos are the same
    fn get_pos_difficulty(&self) -> H256 {
        self.chain()
            .get(&self.tip())
            .unwrap()
            .block
            .header
            .pos_difficulty
    }
    fn get_pow_difficulty(&self, current_ts: BlockTs, parent: H256) -> H256 {
        let dt = |a: i64, b: i64| (a - b) / self.epoch().time;
        let pare = dt(
            self.chain().get(&parent).unwrap().block.header.timestamp,
            self.timestamp(),
        ); // parent epoch
        let cure = dt(current_ts, self.timestamp()); // current epoch
        if cure > pare && self.position().depth > 1 {
            let old_diff: H256 = self
                .chain()
                .get(&parent)
                .unwrap()
                .block
                .header
                .pow_difficulty;
            let mut hash = parent;
            let mut all_hashs = Vec::new();
            loop {
                let block = self.chain().get(&hash).unwrap().block.clone();
                let npow = block.content.reference.clone();
                for pow in npow {
                    if !all_hashs.contains(&pow) {
                        all_hashs.push(pow);
                    }
                }
                hash = self.chain().get(&hash).unwrap().block.header.parent;
                let btime = self.chain().get(&hash).unwrap().block.header.timestamp;

                if dt(btime, self.timestamp()) < pare || btime == self.timestamp() {
                    break;
                }
            }
            //let start_time: u128 = self.chain.get(&hash).unwrap().blk.header.timestamp;
            let ratio = (all_hashs.len() as f64) / (self.epoch().size as f64);
            let new_diff: H256 = old_diff / ratio;
            log::debug!(
                "Mining difficulty changes from {} to {}",
                old_diff,
                new_diff
            );
            new_diff
        } else {
            self.chain()
                .get(&parent)
                .unwrap()
                .block
                .header
                .pow_difficulty
        }
    }
    fn is_block(&self, hash: &H256) -> bool {
        self.chain().contains_key(hash)
    }
    fn is_new_epoch_and_count_blocks(
        &self,
        current_ts: BlockTs,
    ) -> Option<HashMap<Vec<u8>, HashSet<H256>>> {
        let dt = |a: i64, b: i64| (a - b) / self.epoch().time;
        let tip_epoch = dt(
            self.chain()
                .get(&self.tip())
                .unwrap()
                .block
                .header
                .timestamp,
            self.timestamp(),
        );
        let curent_epoch = dt(current_ts, self.timestamp());
        // if it is new epoch, count last epoch's blocks
        let mut tip_iter = self.tip();
        if curent_epoch > tip_epoch {
            let mut cnt: HashMap<Vec<u8>, HashSet<H256>> = HashMap::new();
            loop {
                let b = &self.chain().get(&tip_iter).unwrap().block;
                if b.header.timestamp - self.timestamp() == 0 {
                    break;
                }
                let this_epoch = dt(b.header.timestamp, self.timestamp());
                if this_epoch != tip_epoch {
                    break;
                }
                for h in b.content.reference.iter() {
                    let ref_b = &self
                        .chain()
                        .get(h)
                        .expect("error, transaction ref is not in blockchain!!!")
                        .block;
                    let miner = ref_b.header.vrf_pub_key.clone();
                    if let Some(m) = cnt.get_mut(&miner) {
                        m.insert(*h);
                    } else {
                        let mut m = HashSet::new();
                        m.insert(*h);
                        cnt.insert(miner, m);
                    }
                }
                tip_iter = b.header.parent;
            }
            Some(cnt)
        } else {
            None
        }
    }
    fn print_longest_chain(&self) {
        log::info!("************* Print Longest Chain *************");
        log::info!("{:?}", self.get_all_blocks_from_longest());
        log::info!("***********************************************");
    }
}
