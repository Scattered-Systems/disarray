/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use super::{BlockContent, BlockHeader};
use crate::compute_key_hash;

use scsys::crypto::hash::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BlockType {
    PoS,
    #[default]
    PoW,
}

impl std::convert::From<bool> for BlockType {
    fn from(data: bool) -> Self {
        match data {
            true => Self::PoS,
            false => Self::PoW,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Block {
    pub content: BlockContent,
    pub header: BlockHeader,
    pub block_type: BlockType,
    pub selfish_block: bool,
}

impl Block {
    pub fn new(
        content: BlockContent,
        header: BlockHeader,
        block_type: BlockType,
        selfish_block: bool,
    ) -> Self {
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
