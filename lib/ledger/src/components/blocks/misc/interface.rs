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
use scsys::prelude::{Hashable, H256};

pub trait Resistable {
    fn pos_difficulty(&self) -> H256;
    fn pow_difficulty(&self) -> H256;
}

pub trait Verifiable {
    fn vrf_hash(&self) -> Vec<u8>;
    fn vrf_proof(&self) -> Vec<u8>;
    fn vrf_pub_key(&self) -> Vec<u8>;
}

pub trait BlockHeaderSpec: Hashable + Resistable + Verifiable {
    fn header(&self) -> &Self {
        self
    }
    fn merkle_root(&self) -> H256;
    fn nonce(&self) -> BlockNc;
    fn parent(&self) -> H256;
    // randomness for PoS leader election. TODO: update rand every epoch
    fn rand(&self) -> u128;
    fn timestamp(&self) -> BlockTs;
}

pub trait CoreBlockSpec: Hashable {
    fn content(&self) -> &BlockContent;
    fn header(&self) -> &BlockHeader;
    fn nonce(&self) -> BlockNc {
        self.header().nonce()
    }
    fn parent(&self) -> BlockHs {
        self.header().parent()
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
            let sender = compute_key_hash(txn.sig.pubk);
            let recv = txn.trx.recv;
            log::info!("{:?} sends {:?} value {:?}", sender, recv, txn.trx.value);
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
