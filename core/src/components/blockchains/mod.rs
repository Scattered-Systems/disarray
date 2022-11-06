/*
   Appellation: blockchains <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{blockchain::*, epochs::*, interface::*, misc::*, utils::*};

pub(crate) mod blockchain;
pub(crate) mod epochs;
pub(crate) mod interface;
pub(crate) mod misc;

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

    /// Ouroboros Praos Proof-of-Stake
    pub fn insert_pos(bc: &mut Blockchain, block: &Block, selfish: bool) -> bool {
        //unimplemented!()
        if !selfish {
            if bc.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData = match bc.chain.get(&parenthash) {
                Some(data) => data.clone(),
                None => return false,
            };
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            bc.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            bc.position.pos += 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if newheight > bc.position.depth
                || (newheight == bc.position.depth && block.selfish_block && p < 1.0)
            {
                bc.position.depth = newheight;
                bc.tip = newhash;
                return true;
            }
            false
        } else {
            // Insert a block into blockchain as a selfish miner
            if bc.chain.contains_key(&block.hash()) {
                return false;
            }
            let header: BlockHeader = block.header.clone();
            let parenthash: H256 = header.parent;
            let parentdata: BlockData = match bc.chain.get(&parenthash) {
                Some(data) => data.clone(),
                None => return false,
            };
            let parentheight = parentdata.height;
            let newheight = parentheight + 1;
            let newdata = BlockData::new(block.clone(), newheight);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            bc.chain.insert(newhash, newdata);
            // self.map.insert(newhash, new_mmr);
            bc.position.pos += 1;
            if newheight > bc.position.depth && block.selfish_block {
                bc.lead += 1;
                bc.position.depth = newheight;
                bc.tip = newhash;
                return true;
            } else if !block.selfish_block && newheight > bc.length {
                if bc.lead > 0 {
                    bc.lead -= 1;
                    bc.length += 1;
                    return false;
                } else {
                    bc.position.depth = newheight;
                    bc.tip = newhash;
                    bc.length = newheight;
                    return true;
                }
            }
            false
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
        let parentdata: BlockData = match bc.chain.get(&parenthash) {
            None => return false,
            Some(v) => v.clone(),
        };
        let parentheight = parentdata.height;
        let newheight = parentheight + 1;
        let newdata = BlockData::new(block.clone(), newheight);
        let newhash = block.hash();
        // let mut new_mmr = self.get_mmr(&parenthash);
        // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
        bc.chain.insert(newhash, newdata);
        // self.map.insert(newhash, new_mmr);
        bc.position.pow += 1;

        true
    }
}
