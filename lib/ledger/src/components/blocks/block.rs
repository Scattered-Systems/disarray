/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    BlockContent, BlockHeader, BlockType, CoreBlockSpec, CoreBlockWrapper, CoreBlockWrapperExt,
};
use scsys::{prelude::*, Hashable};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Block {
    pub classification: BlockType,
    pub content: BlockContent,
    pub header: BlockHeader,
    pub selfish_block: bool,
}

impl Block {
    pub fn new(
        classification: BlockType,
        content: BlockContent,
        header: BlockHeader,
        selfish_block: bool,
    ) -> Self {
        Self {
            classification,
            content,
            header,
            selfish_block,
        }
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

impl std::convert::From<Value> for Block {
    fn from(data: Value) -> Self {
        data.into()
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
