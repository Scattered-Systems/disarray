/*
   Appellation: mine <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{DIFFICULTY_PREFIX, BlockId, BlockHs, BlockNc, BlockTs, blocks::{calculate_block_hash, convert_hash_into_binary}};

/// Mines a new block<Dt> where Dt represents transaction data
pub fn create_block_by_mining<Dt: Clone + serde::Serialize>(
    id: BlockId,
    previous: BlockHs,
    timestamp: BlockTs,
    transactions: Vec<Dt>,
) -> (BlockNc, BlockHs) {
    log::info!("Mining a new block...");
    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash = calculate_block_hash(
            id,
            nonce,
            previous.clone(),
            timestamp.clone(),
            transactions.clone(),
        );
        let binary_hash = convert_hash_into_binary(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX.as_ref()) {
            log::info!(
                "mined! nonce: {}, hash: {}, binary hash: {:#?}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash).into());
        }
        nonce += 1;
    }
}
