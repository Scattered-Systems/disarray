/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use crate::{transactions::SignedTransaction, compute_key_hash};
use super::{BlockContent, BlockHeader};
use crate::merkle::MerkleTree;
use scsys::crypto::hash::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BlockClass {
    PoS,
    #[default]
    PoW,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Block {
    pub content: BlockContent,
    pub header: BlockHeader,
    pub block_type: bool,
    pub selfish_block: bool,
}

impl Block {
    pub fn new(content: BlockContent, header: BlockHeader, block_type: bool, selfish_block: bool) -> Self {
        Self {
            content,
            header,
            block_type,
            selfish_block,
        }
    }
    pub fn print_txns(&self) {
        let txns = self.content.data.clone();
        log::info!("***** Print txns in block {:?} *****", self.hash());
        for txn in txns {
            let sender = compute_key_hash(txn.sign.pubk);
            let recv = txn.transaction.recv;
            log::info!(
                "{:?} sends {:?} value {:?}",
                sender,
                recv,
                txn.transaction.value
            );
        }
        log::info!("*************************************");
    }

    pub fn clear_txns(&mut self) {
        self.content.data = Vec::new();
    }
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        hasher(self).as_slice().to_owned().into()
    }
}

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}

pub fn generate_pow_block(
    data: &Vec<SignedTransaction>,
    transaction_ref: &Vec<H256>,
    parent: &H256,
    nonce: u32,
    pow_difficulty: &H256,
    pos_difficulty: &H256,
    timestamp: i64,
    vrf_proof: &Vec<u8>,
    vrf_hash: &Vec<u8>,
    vrf_pub_key: &[u8],
    rand: u128,
    selfish_block: bool,
) -> Block {
    let mt = MerkleTree::new(data);
    let block_type = false;
    let content = BlockContent::new(data.to_vec(), transaction_ref.to_vec());
    let header = BlockHeader {
        parent: *parent,
        nonce: nonce,
        pow_difficulty: *pow_difficulty,
        pos_difficulty: *pos_difficulty,
        timestamp: timestamp,
        merkle_root: mt.root(),
        //mmr_root: parent_mmr.get_merkle_root().unwrap(),
        vrf_proof: vrf_proof.to_vec(),
        vrf_hash: vrf_hash.to_vec(),
        vrf_pub_key: vrf_pub_key.to_vec(),
        rand: rand,
    };
    Block {
        header,
        content,
        block_type,
        selfish_block: selfish_block,
    }
}

pub fn generate_genesis_block(initial_time: i64) -> Block {
    let content = BlockContent::default();
    let block_type = true;
    let selfish_block = false;
    let header = BlockHeader {
        parent: Default::default(),
        nonce: Default::default(),
        //pow_difficulty: <H256>::from([1; 32]),
        pow_difficulty: <H256>::from([
            0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]),
        pos_difficulty: <H256>::from([1; 32]),
        timestamp: initial_time,
        merkle_root: Default::default(),
        // mmr_root: MerkleMountainRange::<Sha256, Vec<Hash>>::new(Vec::new()).get_merkle_root().unwrap(),
        vrf_proof: Default::default(),
        vrf_hash: Default::default(),
        vrf_pub_key: Default::default(),
        rand: Default::default(),
    };
    Block {
        header,
        content,
        block_type,
        selfish_block,
    }
}

// pub fn generate_random_block(parent: &H256,
//     parent_mmr: &MerkleMountainRange<Sha256, Vec<Hash>>) -> Block {
//     let mut rng = rand::thread_rng();
//     let mut data: Vec<SignedTransaction> = Vec::new();
//     let t = generate_random_signed_transaction();
//     data.push(t);
//     let mt: MerkleTree = MerkleTree::new(&data);
//     let content = Content {
//         data: data,
//     };
//     let header = Header {
//         parent: *parent,
//         nonce: rng.gen(),
//         difficulty: hash::generate_random_hash(),
//         timestamp: rng.gen(),
//         merkle_root: mt.root(),
//         mmr_root: parent_mmr.get_merkle_root().unwrap(),
//     };
//     Block {
//         header,
//         content,
//    }
// }

// #[cfg(any(test, test_utilities))]
// pub mod test {
//     use super::*;
//     use crate::crypto::hash::H256;

//     pub fn generate_random_block(parent: &H256,
//         parent_mmr: &MerkleMountainRange<Sha256, Vec<Hash>>) -> Block {
//         let mut rng = rand::thread_rng();
//         let mut data: Vec<SignedTransaction> = Vec::new();
//         let t = generate_random_signed_transaction();
//         data.push(t);
//         let mt: MerkleTree = MerkleTree::new(&data);
//         let content = Content {
//             data: data,
//         };
//         let header = Header {
//             parent: *parent,
//             nonce: rng.gen(),
//             difficulty: hash::generate_random_hash(),
//             timestamp: rng.gen(),
//             merkle_root: mt.root(),
//             mmr_root: parent_mmr.get_merkle_root().unwrap(),
//         };
//         Block {
//             header,
//             content,
//        }
//     }
// }
