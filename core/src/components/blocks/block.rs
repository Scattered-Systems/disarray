/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use super::{content::BlockContent, header::BlockHeader};
use crate::crypto::hash::H256;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BlockClass {
    PoS,
    #[default]
    PoW,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Block {
    pub class: BlockClass,
    pub data: BlockContent,
    pub header: BlockHeader,
}

impl Block {
    pub fn new(class: BlockClass, content: BlockContent, header: BlockHeader) -> Self {
        Self {
            class,
            data: content,
            header,
        }
    }
    pub fn hash(&self) -> H256 {
        let data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec().into()
    }
}

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}
