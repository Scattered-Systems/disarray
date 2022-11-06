/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    blocks::{generate_genesis_block, Block, BlockContent, BlockHeader},
    compute_key_hash,
    transactions::SignedTransaction,
    BlockHs, BlockNc, BlockTs,
};
use scsys::prelude::Hashable;

pub trait CoreBlockSpec: Hashable {
    fn content(&self) -> &BlockContent;
    fn header(&self) -> &BlockHeader;
    fn nonce(&self) -> BlockNc {
        self.header().nonce
    }
    fn parent(&self) -> &BlockHs {
        &self.header().parent
    }
    fn timestamp(&self) -> BlockTs {
        self.header().timestamp
    }
    fn transactions(&self) -> &Vec<SignedTransaction> {
        &self.content().data
    }
}

pub trait CoreBlockWrapper: CoreBlockSpec {
    fn clear_txns(&mut self) -> &Self;
    fn print_txns(&self) -> &Self {
        let txns = self.transactions().clone();
        log::info!("***** Print txns in block {:?} *****", self.hash());
        for txn in txns {
            let sender = compute_key_hash(txn.sign.pubk);
            let recv = txn.transaction.recv;
            log::info!(
                "{:?} sends {:?} value {:?}",
                sender,
                recv,
                txn.transaction.value
            );
        }
        log::info!("*************************************");
        self
    }
}

pub trait CoreBlockWrapperExt: CoreBlockWrapper {
    fn genesis(timestamp: BlockTs) -> Block
    where
        Self: Sized,
    {
        generate_genesis_block(timestamp)
    }
}
