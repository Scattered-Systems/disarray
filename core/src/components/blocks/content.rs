/*
Appellation: block <module>
Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
Description:
    ... Summary ...
*/
use crate::crypto::hash::H256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockContent {
    pub reference: H256,
    pub transactions: Vec<String>
}

impl BlockContent {
    pub fn new(reference: H256, transactions: Vec<String>) -> Self {
        Self { reference, transactions }
    }
}