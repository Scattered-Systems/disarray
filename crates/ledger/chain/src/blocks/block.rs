/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::blocks::{
    BlockContent, BlockHeader, BlockType, CoreBlockSpec, CoreBlockWrapper, CoreBlockWrapperExt,
};
use decanter::prelude::{Hash, Hashable};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Block {
    pub class: BlockType,
    pub content: BlockContent,
    pub header: BlockHeader,
    pub selfish_block: bool,
}

impl Block {
    pub fn new(
        class: BlockType,
        content: BlockContent,
        header: BlockHeader,
        selfish_block: bool,
    ) -> Self {
        Self {
            class,
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
