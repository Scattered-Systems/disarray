/*
   Appellation: blockchains <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{blockchain::*, chain_data::*, pieces::*, utils::*, wrapper::*};

pub(crate) mod blockchain;
pub(crate) mod chain_data;
pub(crate) mod pieces;
pub(crate) mod wrapper;

pub(crate) mod utils {
    use super::{BlockData, Blockchain};
    use crate::blocks::{Block, BlockHeader};
    use scsys::prelude::{
        rand::{self, Rng},
        Hashable, H256,
    };

    // pub fn mmr_push_leaf(mmr: &mut MerkleMountainRange<Sha256, Vec<Hash>>, leaf_hash: Hash) {
    //     let mut leaf_hashes: Vec<Vec<u8>> = mmr
    //         .get_leaf_hashes(0, mmr.get_leaf_count().unwrap() + 1)
    //         .unwrap()
    //         .clone();
    //     leaf_hashes.push(leaf_hash);
    //     mmr.assign(leaf_hashes).unwrap();
    // }

    pub fn insert_pos(bc: &mut Blockchain, block: &Block, selfish: bool) -> bool {
        //unimplemented!()
        if !selfish {
            if bc.chain.contains_key(&block.hash()) {
                panic!("")
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData;
            match bc.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => return false,
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            bc.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            bc.position.pos = bc.position.pos + 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if newheight > bc.position.depth
                || (newheight == bc.position.depth && block.selfish_block == true && p < 1.0)
            {
                bc.position.depth = newheight;
                bc.tip = newhash;
                return true;
            }
            return false;
        } else {
            // Insert a block into blockchain as a selfish miner
            if bc.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData;
            match bc.chain.get(&parenthash) {
                Some(data) => parentdata = data.clone(),
                None => panic!("Failed to find the parent hash in the chain"),
            }
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            bc.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            bc.position.pos = bc.position.pos + 1;
            if newheight > bc.position.depth && block.selfish_block == true {
                bc.lead = bc.lead + 1;
                bc.position.depth = newheight;
                bc.tip = newhash;
                return true;
            } else if block.selfish_block == false && newheight > bc.length {
                if bc.lead > 0 {
                    bc.lead = bc.lead - 1;
                    bc.length = bc.length + 1;
                    return false;
                } else {
                    bc.position.depth = newheight;
                    bc.tip = newhash;
                    bc.length = newheight;
                    return true;
                }
            }
            return false;
        }
    }
    /// Insert a PoW block into blockchain
    pub fn insert_pow(bc: &mut Blockchain, block: &Block) -> bool {
        //unimplemented!()
        if bc.chain.contains_key(&block.hash()) {
            return false;
        }
        let header: BlockHeader = block.header.clone();
        let parenthash: H256 = header.parent;
        let parentdata: BlockData;
        match bc.chain.get(&parenthash) {
            Some(data) => parentdata = data.clone(),
            None => return false,
        }
        let parentheight = parentdata.height;
        let newheight = parentheight + 1;
        let newdata = BlockData::new(block.clone(), newheight);
        let newhash = block.hash();
        // let mut new_mmr = self.get_mmr(&parenthash);
        // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
        bc.chain.insert(newhash, newdata);
        // self.map.insert(newhash, new_mmr);
        bc.position.pow = bc.position.pow + 1;

        return true;
    }
}
