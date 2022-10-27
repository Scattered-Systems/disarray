/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use super::{content::BlockContent, header::BlockHeader};
use crate::{crypto::hash::H256, transactions::Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};


#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Block {
    pub content: BlockContent,
    pub header: BlockHeader,
}

impl Block {
    pub fn new(content: BlockContent, header: BlockHeader) -> Self {
        Self { content, header }
    }
    pub fn hash(&self) -> H256 {
        let data = json!(self.header);
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().to_vec().into()
    }
}

impl std::convert::Into<Value> for Block {
    fn into(self) -> Value {
        json!(self)
    }
}