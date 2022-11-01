/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    interface::{CoreBlockSpec, CoreBlockWrapper, CoreBlockWrapperExt},
    BlockContent, BlockHeader,
};
use scsys::prelude::{Hashable, H256};
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
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        blake3::hash(serde_json::to_string(&self).unwrap().as_bytes())
            .as_bytes()
            .to_owned()
            .into()
    }
}

impl CoreBlockSpec for Block {
    fn content(&self) -> &BlockContent {
        &self.content
    }
    fn header(&self) -> &BlockHeader {
        &self.header
    }
}

impl CoreBlockWrapper for Block {
    fn clear_txns(&mut self) -> &Self {
        self.content.data = Vec::new();
        self
    }
}

impl CoreBlockWrapperExt for Block {}

// impl std::convert::Into<String> for Block {
//     fn into(self) -> String {
//         serde_json::to_string(&self).expect("Failed to serialize the content...")
//     }
// }

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "",)
    }
}
