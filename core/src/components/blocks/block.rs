/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use super::{content::BlockContent, header::BlockHeader};
use crate::crypto::hash::{Hashable, H256, hasher};
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
    pub data: BlockContent,
    pub header: BlockHeader,
}

impl Block {
    pub fn new(class: BlockClass, data: BlockContent, header: BlockHeader) -> Self {
        Self {class, data, header }
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
