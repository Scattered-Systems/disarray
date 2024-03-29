/*
   Appellation: miners <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{channels::*, context::*, miner::*, pools::*, signals::*, utils::*};

pub(crate) mod channels;
pub(crate) mod context;
pub(crate) mod miner;
pub(crate) mod pools;
pub(crate) mod signals;

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
