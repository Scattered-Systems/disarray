/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{BlockContent, BlockHeader};
use crate::compute_key_hash;

use algae::merkle::{MerkleTree, MerkleTreeWrapper};
use scsys::{core::Timestamp, prelude::{rand::{self, Rng}, Hashable, H256, generate_random_hash}};
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
    pub fn clear_txns(&mut self) {
        self.content.data = Vec::new();
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
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        blake3::hash(serde_json::to_string(&self).unwrap().as_bytes()).as_bytes().to_owned().into()
    }
}

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "", )
    }
}
