/*
   Appellation: blocks <utils>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/

use crate::{blocks::Block, DIFFICULTY_PREFIX};
use scsys::core::{BlockHs, BlockId, BlockNc, BlockTs};
use sha2::Digest;

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

pub fn determine_chain_validity(chain: &[crate::blocks::Block]) -> bool {
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

pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res.into_bytes()
}

pub fn calculate_block_hash<Dt: Clone + serde::Serialize>(
    id: BlockId,
    nonce: BlockNc,
    previous: BlockHs,
    timestamp: BlockTs,
    transactions: Vec<Dt>,
) -> Vec<u8> {
    let cache = serde_json::json!(
        {
            "id": id,
            "nonce": nonce,
            "previous": previous,
            "timestamp": timestamp,
            "transactions": transactions.clone()
        }
    );
    let mut hasher = sha2::Sha256::new();
    hasher.update(cache.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}
