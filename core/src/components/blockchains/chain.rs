use crate::{blocks::{generate_genesis_block, Block, BlockHeader as Header}, BlockTs};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use log::{debug, info};
use tari_mmr::{MerkleMountainRange, MerkleProof};
use scsys::{
    crypto::hash::{hash_divide_by, Hashable, H256},
    prelude::{rand::{self, Rng}},
};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Data {
    blk: Block,
    height: u128,
}

pub struct Blockchain {
    chain: HashMap<H256, Data>,
    map: HashMap<H256, MerkleMountainRange<Sha256, Vec<H256>>>,
    tip: H256,
    depth: u128,
    num_pos: u128,
    num_pow: u128,
    epoch_size: u128,
    epoch_time: BlockTs,
    genesis_time: BlockTs,
    pub_len: u128,
    private_lead: u128,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new(initial_time: BlockTs) -> Self {
        //unimplemented!()
        let genesis = generate_genesis_block(initial_time);
        info!(
            "Timestamp of the genesis block: {}",
            genesis.header.timestamp
        );
        let blockinfo = Data {
            blk: genesis.clone(),
            height: 0,
        };
        let hash: H256 = genesis.clone().hash();
        let mut chain = HashMap::new();
        chain.insert(hash, blockinfo);
        let mut map = HashMap::new();
        map.insert(
            hash,
            MerkleMountainRange::new(Vec::new()),
        );
        let tip: H256 = hash;
        //info!("0:{}",tip);
        Blockchain {
            chain,
            map,
            tip,
            depth: 0,
            num_pos: 0,
            num_pow: 0,
            epoch_size: 400,
            epoch_time: 120_000_000,
            genesis_time: initial_time,
            pub_len: 0,
            private_lead: 0,
        }
    }

    /// Insert a PoS block into blockchain
    pub fn insert_pos(&mut self, block: &Block, selfish: bool) -> bool {
        //unimplemented!()
        if !selfish {
            if self.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: Header = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: Data;
            match self.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => return false,
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = Data {
                blk: block.clone(),
                height: newheight,
            };
            let newhash = block.hash();
            let mut new_mmr = self.get_mmr(&parenthash);
            mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, newdata);
            self.map.insert(newhash, new_mmr);
            self.num_pos = self.num_pos + 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if newheight > self.depth
                || (newheight == self.depth && block.selfish_block == true && p < 1.0)
            {
                self.depth = newheight;
                self.tip = newhash;
                return true;
            }
            return false;
        } else {
            /// Insert a block into blockchain as a selfish miner
            if self.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: Header = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: Data;
            match self.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => return false,
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = Data {
                blk: block.clone(),
                height: newheight,
            };
            let newhash = block.hash();
            let mut new_mmr = self.get_mmr(&parenthash);
            mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            self.chain.insert(newhash, newdata);
            self.map.insert(newhash, new_mmr);
            self.num_pos = self.num_pos + 1;
            if newheight > self.depth && block.selfish_block == true {
                self.private_lead = self.private_lead + 1;
                self.depth = newheight;
                self.tip = newhash;
                return true;
            } else if block.selfish_block == false && newheight > self.pub_len {
                if self.private_lead > 0 {
                    self.private_lead = self.private_lead - 1;
                    self.pub_len = self.pub_len + 1;
                    return false;
                } else {
                    self.depth = newheight;
                    self.tip = newhash;
                    self.pub_len = newheight;
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
        let header: Header = block.header.clone();
        let parenthash: H256 = header.parent;
        let parentdata: Data;
        match self.chain.get(&parenthash) {
            Some(data) => parentdata = data.clone(),
            None => return false,
        }
        let parentheight = parentdata.height;
        let newheight = parentheight + 1;
        let newdata = Data {
            blk: block.clone(),
            height: newheight,
        };
        let newhash = block.hash();
        let mut new_mmr = self.get_mmr(&parenthash);
        mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
        self.chain.insert(newhash, newdata);
        self.map.insert(newhash, new_mmr);
        self.num_pow = self.num_pow + 1;

        return true;
    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        //unimplemented!()
        self.tip
    }

    pub fn get_pow_difficulty(&self, current_ts: BlockTs, parent: H256) -> H256 {
        let epoch_size = self.epoch_size;
        let depth = self.depth;
        let epoch_time = self.epoch_time;
        let parent_time = self.chain.get(&parent).unwrap().blk.header.timestamp;
        let genesis_time = self.genesis_time;
        let parent_epoch = (parent_time - genesis_time) / epoch_time;
        let curent_epoch = (current_ts - genesis_time) / epoch_time;
        if curent_epoch > parent_epoch && depth > 1 {
            let old_diff: H256 = self.chain.get(&parent).unwrap().blk.header.pow_difficulty;
            let mut hash = parent.clone();
            let mut all_hashs = Vec::new();
            while true {
                let blk = self.chain.get(&hash).unwrap().blk.clone();
                let pow_blks = blk.content.reference.clone();
                for pow_blk in pow_blks {
                    if !all_hashs.contains(&pow_blk) {
                        all_hashs.push(pow_blk);
                    }
                }
                hash = self.chain.get(&hash).unwrap().blk.header.parent;
                let blk_time = self.chain.get(&hash).unwrap().blk.header.timestamp;
                let blk_epoch = (blk_time - genesis_time) / epoch_time;
                if blk_epoch < parent_epoch || blk_time == self.genesis_time {
                    break;
                }
            }
            let num_blk = all_hashs.len();
            //let start_time: u128 = self.chain.get(&hash).unwrap().blk.header.timestamp;
            let mut ratio = (num_blk as f64) / (epoch_size as f64);
            //println!("Ratio: {}", ratio);
            // if ratio > 4.0 {
            // 	ratio = 4.0;
            // } else if ratio < 0.25 {
            // 	ratio = 0.25;
            // }
            let new_diff: H256 = hash_divide_by(&old_diff, ratio);
            debug!(
                "Mining difficulty changes from {} to {}",
                old_diff, new_diff
            );
            new_diff
        } else {
            self.chain.get(&parent).unwrap().blk.header.pow_difficulty
        }
    }

    pub fn epoch(&self, current_ts: BlockTs) -> BlockTs {
        let epoch_size = self.epoch_size;
        //let depth = self.depth;
        let epoch_time = self.epoch_time;
        //let tip = self.tip;
        //let tip_time = self.chain.get(&tip).unwrap().blk.header.timestamp;
        let genesis_time = self.genesis_time;
        //let tip_epoch = (tip_time - genesis_time)/epoch_time;
        let current_epoch = (current_ts - genesis_time) / epoch_time;
        current_epoch
    }

    pub fn is_new_epoch_and_count_blocks(
        &self,
        current_ts: BlockTs,
    ) -> Option<HashMap<Vec<u8>, HashSet<H256>>> {
        let epoch_size = self.epoch_size;
        let depth = self.depth;
        let epoch_time = self.epoch_time;
        let tip = self.tip;
        let tip_time = self.chain.get(&tip).unwrap().blk.header.timestamp;
        let genesis_time = self.genesis_time;
        let tip_epoch = (tip_time - genesis_time) / epoch_time;
        let curent_epoch = (current_ts - genesis_time) / epoch_time;
        // if it is new epoch, count last epoch's blocks
        let mut tip_iter = tip;
        if curent_epoch > tip_epoch {
            let mut cnt: HashMap<Vec<u8>, HashSet<H256>> = HashMap::new();
            loop {
                let b = &self.chain.get(&tip_iter).unwrap().blk;
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
                        .blk;
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

    pub fn get_pos_difficulty(&self) -> H256 {
        // should be parent, but it's okay since all pos are the same
        self.chain.get(&self.tip).unwrap().blk.header.pos_difficulty
    }

    pub fn get_depth(&self) -> u128 {
        self.depth
    }

    pub fn get_num_pos(&self) -> u128 {
        self.num_pos
    }

    pub fn get_num_pow(&self) -> u128 {
        self.num_pow
    }

    pub fn get_size(&self) -> usize {
        self.chain.len()
    }

    pub fn get_pub_len(&self) -> u128 {
        self.pub_len
    }

    pub fn get_lead(&self) -> u128 {
        self.private_lead
    }

    pub fn get_mmr(&self, hash: &H256) -> MerkleMountainRange<Sha256, Vec<Hash>> {
        let mmr_ref = self.map.get(hash).unwrap();
        let leaf_hashes = mmr_ref
            .get_leaf_hashes(0, mmr_ref.get_leaf_count().unwrap() + 1)
            .unwrap()
            .clone();
        let mut mmr_ret = MerkleMountainRange::<Sha256, Vec<Hash>>::new(Vec::new());
        mmr_ret.assign(leaf_hashes).unwrap();
        mmr_ret
    }

    pub fn contains_hash(&self, hash: &H256) -> bool {
        self.chain.contains_key(hash)
    }

    pub fn print_longest_chain(&self) {
        let mut longest_chain = self.all_blocks_in_longest_chain();
        info!("************* Print Longest Chain *************");
        info!("{:?}", longest_chain);
        info!("***********************************************");
    }

    /// Get the last block's hash of the longest chain
    //#[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        //unimplemented!()
        let mut all_block: Vec<H256> = vec![];
        let mut current_hash = self.tip;
        //let mut parent_hash;
        let mut parentdata: Data;

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => parentdata = data.clone(),
            }
            all_block.push(current_hash);
            current_hash = parentdata.blk.header.parent;
            // debug!("current_hash {:?}!", current_hash);
            // debug!("contains {:?}!", self.chain.get(&current_hash));
        }
        debug!("finish {:?}!", all_block);

        all_block.reverse();
        all_block
    }

    pub fn find_one_height(&self, height: u128) -> H256 {
        let mut current_hash = self.tip;
        //let parent_hash: H256 = hash.clone();
        let mut childdata: Data;

        loop {
            childdata = self.chain.get(&current_hash).unwrap().clone();
            if childdata.height == height {
                return childdata.blk.hash().clone();
            }
            current_hash = childdata.blk.header.parent.clone();
        }
    }

    pub fn get_longest_chain(&self) -> Vec<Block> {
        //unimplemented!()
        let mut all_block: Vec<H256> = vec![];
        let mut current_hash = self.tip;
        //let mut parent_hash;
        let mut parentdata: Data;

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => parentdata = data.clone(),
            }
            all_block.push(current_hash);
            current_hash = parentdata.blk.header.parent;
            // debug!("current_hash {:?}!", current_hash);
            // debug!("contains {:?}!", self.chain.get(&current_hash));
        }
        all_block.reverse();
        debug!("finish {:?}!", all_block);

        let mut chain: Vec<Block> = vec![];
        for hash in all_block {
            chain.push(self.find_one_block(&hash).unwrap().clone());
        }
        chain
    }

    pub fn get_chain_quality(&self) -> f32 {
        //unimplemented!()
        // let mut all_block : Vec<H256> = vec![];
        let mut current_hash = self.tip;
        let mut parentdata: Data;
        let mut count = 0;
        let mut count_selfish = 0;
        let mut all_pow_hash: Vec<H256> = Vec::new();

        loop {
            match self.chain.get(&current_hash) {
                None => break,
                Some(data) => parentdata = data.clone(),
            }
            //all_block.push(current_hash);
            let pow_hashes = parentdata.blk.content.reference.clone();
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

            current_hash = parentdata.blk.header.parent;
        }
        let chain_quality: f32 = 1.0 - (count_selfish as f32) / (count as f32);
        chain_quality
    }

    pub fn find_one_block(&self, hash: &H256) -> Option<Block> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.blk.clone()),
        }
    }

    pub fn find_one_header(&self, hash: &H256) -> Option<Header> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.blk.header.clone()),
        }
    }
    pub fn find_one_depth(&self, hash: &H256) -> Option<u128> {
        match self.chain.get(&hash) {
            None => return None,
            Some(data) => return Some(data.height),
        }
    }
}

pub fn mmr_push_leaf(mmr: &mut MerkleMountainRange<Sha256, Vec<Hash>>, leaf_hash: Hash) {
    let mut leaf_hashes: Vec<Vec<u8>> = mmr
        .get_leaf_hashes(0, mmr.get_leaf_count().unwrap() + 1)
        .unwrap()
        .clone();
    leaf_hashes.push(leaf_hash);
    mmr.assign(leaf_hashes).unwrap();
}

// FlyClientProposal is a proposal sent from the prover,
// it contains current chain depth and last block header.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct FlyClientProposal {
    pub chain_depth: usize,
    pub header: Header,
}

impl FlyClientProposal {
    pub fn new(blockchain: &Blockchain) -> Self {
        FlyClientProposal {
            chain_depth: blockchain.depth as usize,
            header: blockchain.find_one_block(&blockchain.tip()).unwrap().header,
        }
    }
}

// FlyClientQuery is the query sent from verifier to prover,
// it contains the chain depth of a proposal and a sample of
// blocks for proof. Note sample points are < query_depth - 1.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct FlyClientQuery {
    pub query_depth: usize,
    pub sample: Vec<usize>,
}

impl FlyClientQuery {
    pub fn new(proposal_depth: usize, sample: Vec<usize>) -> Self {
        FlyClientQuery {
            query_depth: proposal_depth,
            sample,
        }
    }
}

// The proof for a single point provided by the prover. To handle
// all the sample of a query, need a Vec<FlyClientProof>.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct FlyClientProof {
    // leaf_idx is corresponding to a number in the query sample
    leaf_idx: usize,
    // block header corresponding to the leaf_idx
    pub leaf_header: Header,
    // merkle proof for this block
    merkle_proof: MerkleProof,
}

impl FlyClientProof {
    // query depth is from the FlyClientQuery
    pub fn new(blockchain: &Blockchain, leaf_idx: usize, query_depth: usize) -> Self {
        // Note get_longest_chain() include genesis block with is not included in depth.
        let leaf_hash: H256 = blockchain.get_longest_chain()[leaf_idx + 1].hash();
        let leaf_header = blockchain.find_one_block(&leaf_hash).unwrap().header;
        let mmr_hash = blockchain.get_longest_chain()[query_depth - 2 + 1].hash();
        let mmr = blockchain.get_mmr(&mmr_hash);
        let merkle_proof = MerkleProof::for_leaf_node(&mmr, leaf_idx).unwrap();
        FlyClientProof {
            leaf_idx,
            leaf_header,
            merkle_proof,
        }
    }

    // only deals with first two step verification in the paper.
    pub fn verify(self, mmr_root: Hash) -> bool {
        assert!(self
            .merkle_proof
            .verify_leaf::<Sha256>(
                &mmr_root[..],
                self.leaf_header.hash().as_ref(),
                self.leaf_idx
            )
            .is_ok());
        true
    }
}

// #[cfg(any(test, test_utilities))]
// mod tests {
//     use super::*;
//     use crate::block::test::generate_random_block;
//     use crate::crypto::hash::Hashable;

//     #[test]
//     fn blockchain_mmr_test() {
//         let mut blockchain = Blockchain::new();
// 		let genesis_hash = blockchain.tip();
// 		let genesis_mmr = blockchain.get_mmr(&genesis_hash);
//         let block = generate_random_block(&genesis_hash, &genesis_mmr);
//         blockchain.insert(&block);
// 		assert_eq!(blockchain.tip(), block.hash());
// 		let tip_mmr = blockchain.get_mmr(&blockchain.tip);
// 		println!("{} {}", tip_mmr.get_leaf_count().unwrap(), tip_mmr.len().unwrap());
// 		assert!(MerkleProof::for_leaf_node(&tip_mmr, 0).is_ok());

// 		let block_hash = blockchain.tip();
// 		let block_mmr = blockchain.get_mmr(&block_hash);
//         let block1 = generate_random_block(&block_hash, &block_mmr);
//         blockchain.insert(&block1);
// 		assert_eq!(blockchain.tip(), block1.hash());
// 		let tip_mmr = blockchain.get_mmr(&blockchain.tip);
// 		println!("{} {}", tip_mmr.get_leaf_count().unwrap(), tip_mmr.len().unwrap());
// 		assert!(MerkleProof::for_leaf_node(&tip_mmr, 1).is_ok());

// 		let block1_hash = blockchain.tip();
// 		let block1_mmr = blockchain.get_mmr(&block1_hash);
//         let block2 = generate_random_block(&block1_hash, &block1_mmr);
//         blockchain.insert(&block2);
// 		assert_eq!(blockchain.tip(), block2.hash());
// 		let tip_mmr = blockchain.get_mmr(&blockchain.tip);
// 		println!("{} {}", tip_mmr.get_leaf_count().unwrap(), tip_mmr.len().unwrap());
// 		assert!(MerkleProof::for_leaf_node(&tip_mmr, 2).is_ok());

// 		let block2_hash = blockchain.tip();
// 		let block2_mmr = blockchain.get_mmr(&block2_hash);
//         let block3 = generate_random_block(&block2_hash, &block2_mmr);
//         blockchain.insert(&block3);
// 		assert_eq!(blockchain.tip(), block3.hash());
// 		let tip_mmr = blockchain.get_mmr(&blockchain.tip);
// 		println!("{} {}", tip_mmr.get_leaf_count().unwrap(), tip_mmr.len().unwrap());
// 		assert!(MerkleProof::for_leaf_node(&tip_mmr, 3).is_ok());

// 		let proposal: FlyClientProposal = FlyClientProposal::new(&blockchain);
// 		let query: FlyClientQuery = FlyClientQuery::new(proposal.chain_depth, vec![0]);
// 		let proof: FlyClientProof = FlyClientProof::new(&blockchain, 0, query.query_depth);
// 		assert!(proof.verify(proposal.header.mmr_root));
//     }
// }
