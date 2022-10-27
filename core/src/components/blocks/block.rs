/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use super::{content::BlockContent, header::BlockHeader};
use crate::{crypto::hash::{hasher, Hashable, H256}, compute_key_hash};
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
    pub class: BlockClass,
    pub content: BlockContent,
    pub header: BlockHeader,
}

impl Block {
    pub fn new(class: BlockClass, content: BlockContent, header: BlockHeader) -> Self {
        Self {
            class,
            content,
            header,
        }
    }
    pub fn print_txns(&self) {
        let txns = self.content.data.clone();
        log::info!("***** Print txns in block {:?} *****", self.hash());
        for txn in txns {
            let sender = compute_key_hash(txn.sign.pubk);
            let recv = txn.transaction.recv;
            log::info!("{:?} sends {:?} value {:?}", sender, recv, txn.transaction.value);
        }
        log::info!("*************************************");
    }

    pub fn clear_txns(&mut self) {
        self.content.data = Vec::new();
    }
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}
