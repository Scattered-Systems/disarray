/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{blocks::Block, compute_key_hash, BlockState, StateMap};
use scsys::prelude::{Hashable, H160, H256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub state_per_block: BlockState
}

impl State {
    pub fn new() -> Self {
        let state_per_block = BlockState::new();
        State { state_per_block }
    }

    pub fn ico(&mut self, genesis_hash: H256, accounts: &Vec<H160>, amount: usize) {
        if !self.state_per_block.is_empty() {
            tracing::info!("Already did an ICO!");
            return;
        }
        let mut s = StateMap::new();
        for account in accounts {
            s.insert(*account, (0, amount));
        }
        self.state_per_block.insert(genesis_hash, s);
    }

    pub fn update_block(&mut self, block: &Block) {
        if self.state_per_block.contains_key(&block.hash()) {
            return;
        }
        let parent_hash = block.header.parent;
        if !self.state_per_block.contains_key(&parent_hash) {
            tracing::info!("Updating a block before its parent!");
            return;
        }
        let mut parent_state = self.state_per_block.get(&parent_hash).unwrap().clone();
        let txns = block.content.data.clone();
        for txn in txns {
            let sender: H160 = compute_key_hash(txn.sign.pubk).into();
            let recv = txn.transaction.recv;
            // todo: add txn check, if the account exists or amount is enough

            let (s_nonce, s_amount) = *parent_state.get(&sender).unwrap();
            let (r_nonce, r_amount) = *parent_state.get(&recv).unwrap();
            parent_state.insert(sender, (s_nonce + 1, s_amount - txn.transaction.value));
            parent_state.insert(recv, (r_nonce, r_amount + txn.transaction.value));
        }
        self.state_per_block.insert(block.hash(), parent_state);
    }

    pub fn update_blocks(&mut self, blocks: &Vec<Block>) {
        for block in blocks {
            self.update_block(block);
        }
    }

    pub fn check_block(&mut self, hash: &H256) -> bool {
        self.state_per_block.contains_key(hash)
    }

    pub fn one_block_state(&mut self, hash: &H256) -> StateMap {
        let find_state = self.state_per_block.get(hash).unwrap().clone();
        find_state
    }

    pub fn print_last_block_state(&mut self, hash: &H256) {
        let last_state = self.state_per_block.get(hash).unwrap().clone();
        for (key, value) in last_state {
            let (nonce, amount) = value;
            tracing::info!("account {:?} has nonce {} value {}", key, nonce, amount);
        }
    }
}

pub trait Stateful {
    fn state(&self) -> &Self {
        &self
    }
}
