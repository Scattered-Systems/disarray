/*
   Appellation: blockchains <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{attr::*, blockchain::*, utils::*};

pub(crate) mod attr;
pub(crate) mod blockchain;

pub(crate) mod utils {
    use crate::{
        blockchains::{BlockData, Blockchain, ChainWrapperExt},
        blocks::{Block, BlockHeader, BlockHeaderSpec, CoreBlockSpec},
    };
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

    pub trait OuroborosPraos {
        fn insert_selfish_pos(&self, blockchain: &mut Blockchain, block: &Block) -> bool {
            insert_selfish_pos(blockchain, block)
        }
        fn insert_unselfish_pos(&self, blockchain: &mut Blockchain, block: &Block) -> bool {
            insert_unselfish_pos(blockchain, block)
        }
        fn insert(&self, blockchain: &mut Blockchain, block: &Block, selfish: bool) -> bool {
            if !selfish {
                self.insert_unselfish_pos(blockchain, block)
            } else {
                self.insert_selfish_pos(blockchain, block)
            }
        }
    }

    pub fn insert_selfish_pos(bc: &mut Blockchain, block: &Block) -> bool {
        if bc.chain.contains_key(&block.hash()) {
            false
        } else {
            let header: BlockHeader = block.header().clone();
            let parenthash: H256 = header.parent();
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

    pub fn insert_unselfish_pos(bc: &mut Blockchain, block: &Block) -> bool {
        if bc.chain.contains_key(&block.hash()) {
            false
        } else {
            let pdata: BlockData = match bc.find_one_payload(&block.header.parent()) {
                Some(v) => v,
                None => return false,
            };
            let height = pdata.height + 1;
            let data = BlockData::new(block.clone(), height);
            let newhash = block.hash();
            // let mut new_mmr = self.get_mmr(&parenthash);
            // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
            bc.chain.insert(newhash, data);
            // self.map.insert(newhash, new_mmr);
            bc.position.pos += 1;

            let mut rng = rand::thread_rng();
            let p: f64 = rng.gen::<f64>(); // toss a coin

            if height > bc.position.depth
                || (height == bc.position.depth && block.selfish_block && p < 1.0)
            {
                bc.position.depth = height;
                bc.tip = newhash;
                return true;
            }
            false
        }
    }

    /// Ouroboros Praos Proof-of-Stake
    pub fn insert_pos(bc: &mut Blockchain, block: &Block, selfish: bool) -> bool {
        //unimplemented!()
        if !selfish {
            insert_unselfish_pos(bc, block)
        } else {
            insert_selfish_pos(bc, block)
        }
    }
    /// Insert a PoW block into blockchain
    pub fn insert_pow(bc: &mut Blockchain, block: &Block) -> bool {
        //unimplemented!()
        if bc.is_block(&block.hash()) {
            return false;
        }
        let prev: BlockData = match bc.find_one_payload(&block.header().parent()) {
            None => return false,
            Some(v) => v,
        };
        let data = BlockData::new(block.clone(), prev.height + 1);
        let hash = block.hash();
        // let mut new_mmr = self.get_mmr(&parenthash);
        // mmr_push_leaf(&mut new_mmr, newhash.as_ref().to_vec().clone());
        bc.chain.insert(hash, data);
        // self.map.insert(newhash, new_mmr);
        bc.position.pow += 1;

        true
    }
}
