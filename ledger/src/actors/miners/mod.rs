/*
   Appellation: miners <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{channels::*, context::*, miner::*, signals::*, utils::*};

pub(crate) mod channels;
pub(crate) mod context;
pub(crate) mod miner;

pub(crate) mod signals {
    use crate::ContextUpdateSignal;
    use crossbeam::channel::{unbounded, Receiver, Sender};

    /// Bootstrap's a channel reciever and sender together under one structure
    pub struct SignalPack<T = ContextUpdateSignal> {
        pub receiver: Receiver<T>,
        pub sender: Sender<T>,
    }

    impl<T> SignalPack<T> {
        pub fn new(receiver: Receiver<T>, sender: Sender<T>) -> Self {
            Self { receiver, sender }
        }
        pub fn unbounded() -> Self {
            let (sender, recv) = unbounded();
            Self::new(recv, sender)
        }
    }

    impl<T> Default for SignalPack<T> {
        fn default() -> Self {
            Self::unbounded()
        }
    }
}

pub(crate) mod utils {
    use crate::{
        blocks::calculate_block_hash, transactions::SignedTransaction, BlockHs, BlockId, BlockNc,
        BlockTs, DIFFICULTY_PREFIX,
    };

    /// Mines a new block<Dt> where Dt represents transaction data
    pub fn create_block_by_mining(
        id: BlockId,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<SignedTransaction>,
    ) -> (BlockNc, BlockHs) {
        log::info!("Mining a new block...");
        let mut nonce = 0;
        loop {
            if nonce % 100000 == 0 {
                log::info!("nonce: {}", nonce);
            }
            let hash = calculate_block_hash(id, nonce, previous, timestamp, transactions.clone());
            let binary_hash = &hash.0;
            if binary_hash.starts_with(DIFFICULTY_PREFIX.as_ref()) {
                log::info!(
                    "mined! nonce: {}, hash: {}, binary hash: {:#?}",
                    nonce,
                    hex::encode(hash),
                    binary_hash
                );
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
