/*
    Appellation: messages <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{blocks::Block, transactions::*};
use scsys::prelude::H256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Message {
    Ping(String),
    Pong(String),
    NewBlockHashes(Vec<H256>),
    GetBlocks(Vec<H256>),
    Blocks(Vec<Block>),
    NewTransactionHashes(Vec<H256>),
    GetTransactions(Vec<H256>),
    Transactions(Vec<SignedTransaction>),
    // spv client
    SPVGetChain(),
    SPVChain(Vec<Block>),
    SPVVerifyTxn(H256, H256),
    SPVVerifyRandomTxn(),
    SPVTxnProof(H256, H256, H256, Vec<H256>, usize, usize),
    //fly client
    FlyGetChain(),
    // FlyChain(FlyClientProposal,FlyClientProof),
    FlyVerifyRandomTxn(),
    // FlyTxnProof(FlyClientProposal, FlyClientProof, H256, Vec<H256>, usize, usize,H256)
}