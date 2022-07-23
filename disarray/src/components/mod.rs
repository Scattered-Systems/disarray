/*
   Appellation: actors <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use blocks::*;
pub use chains::*;
pub use utils::*;

mod blocks;
mod chains;

mod utils {
    use crate::{calculate_block_hash, convert_hash_into_binary, Block, DIFFICULTY_PREFIX};

    /// Determine the validity of a new block by comparing the previous one
    pub fn determine_block_validity(block: &Block, pblock: &Block) -> bool {
        if block.previous != pblock.hash {
            log::warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !convert_hash_into_binary(
            &hex::decode(&block.hash).expect("Decoding Error: failed to decode the BlockHash"),
        )
            .starts_with(DIFFICULTY_PREFIX.as_ref())
        {
            log::warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != pblock.id + 1 {
            log::warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id,
                pblock.id
            );
            return false;
        } else if hex::encode(calculate_block_hash(
            block.id,
            block.nonce,
            block.previous.clone(),
            block.timestamp.clone(),
            block.transactions.clone(),
        )) != block.hash
        {
            log::warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    pub fn determine_chain_validity(chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !determine_block_validity(second, first) {
                return false;
            }
        }
        true
    }
}
