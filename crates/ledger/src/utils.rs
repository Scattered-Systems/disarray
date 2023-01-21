/*
   Appellation: utils <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{
    blocks::{Block, BlockHeader, BlockHeaderSpec, CoreBlockSpec},
    transactions::{verify_signedtxn, SignedTransaction},
    BlockData, Blockchain, ChainWrapperExt, Merger, StateMap,
};
use ckb_merkle_mountain_range::util::MemMMR;
use decanter::prelude::{Hashable, H160, H256};
use rand::Rng;
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::io::{self, BufRead, BufReader};

/// Creates a vector of accounts from the provided collection of keys
pub fn create_ico_accounts(keys: Vec<Ed25519KeyPair>) -> Vec<H160> {
    keys.iter()
        .map(|i| compute_key_hash(i.public_key().as_ref().to_vec()).into())
        .collect::<Vec<H160>>()
}
/// Creates a vector of the given size composed of elligble keypairs
pub fn create_ico_keys(n: usize) -> Vec<Ed25519KeyPair> {
    let lines: Vec<String> = file_to_vec("pubkeys.txt".to_string()).unwrap();

    let mut keys: Vec<Ed25519KeyPair> = Vec::new();
    for i in lines.iter().take(n) {
        let pkcs8_bytes = hex::decode(i.clone()).unwrap();
        let key = Ed25519KeyPair::from_pkcs8(&pkcs8_bytes[..]).unwrap();
        keys.push(key);
    }
    keys
}
///
pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{c:b}"));
    }
    res.into_bytes()
}
/// A function wrapper converting the given vector of elements type u8
pub fn compute_key_hash(key: Vec<u8>) -> H256 {
    key.into()
}
/// From the given path, open the file and gathers its contents into a vector
pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = std::fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
/// Function wrapper for adding an new leaf to the owned [MemMMR]
pub fn mmr_push_leaf(mmr: &mut MemMMR<H256, Merger>, leaf_hash: H256) {
    mmr.push(leaf_hash)
        .expect("Failed to add the leaf to the merkle mountain range...");
}

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
/// Check the given transaction against the provided state-map
pub fn transaction_check(current_state: &mut StateMap, tx: &SignedTransaction) -> bool {
    if verify_signedtxn(tx) {
        let copy = tx.clone();
        let pubk = copy.sig.pubk.clone();
        let nonce = copy.trx.nonce;
        let value = copy.trx.value;
        let recv = copy.trx.recv;

        let sender: H160 = compute_key_hash(pubk).into();
        let (s_nonce, s_amount) = *current_state.get(&sender).unwrap();
        let (r_nonce, r_amount) = *current_state.get(&recv).unwrap();

        if nonce == s_nonce + 1 && s_amount >= value {
            current_state.insert(sender, (s_nonce + 1, s_amount - value));
            current_state.insert(recv, (r_nonce, r_amount + value));
            true
        } else {
            false
        }
    } else {
        false
    }
}
