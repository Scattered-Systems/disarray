/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
extern crate disarray_chain;

use disarray_chain::{blocks::Block, transactions::SignedTransaction};

use decanter::prelude::{Hash, Hashable, H256};
use scsys::prelude::fnl_remove;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum ControlSignal {
    ConnectNewPeer(ConnectRequest),
    BroadcastMessage(Message),
}

#[derive(Clone, Debug)]
pub struct ConnectRequest {
    pub addr: std::net::SocketAddr,
    // pub result_chan: crossbeam::channel::Sender<std::io::Result<BaseHandle>>,
}

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

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(&self).ok().unwrap())
        )
    }
}
