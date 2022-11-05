/*
   Appellation: blocks <validators>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blocks::Block, convert_hash_into_binary, DIFFICULTY_PREFIX};

/// Determine the validity of a new block by comparing the previous one
pub fn determine_block_validity(block: &Block, pblock: &Block) -> bool {
    let _curblock = block.clone();
    let _prev = pblock.clone();
    true
}

// pub fn determine_block_validity(block: &Block, pblock: &Block) -> bool {
//     if block.content.reference != pblock.content.reference {
//         log::warn!("block with id: {} has wrong previous hash", block.header.id);
//         return false;
//     } else if !convert_hash_into_binary(
//         &hex::decode(&block.content.reference).expect("Decoding Error: failed to decode the BlockHash"),
//     )
//     .starts_with(DIFFICULTY_PREFIX.as_ref())
//     {
//         log::warn!("block with id: {} has invalid difficulty", block.header.id);
//         return false;
//     } else if block.header.id != pblock.header.id + 1 {
//         log::warn!(
//             "block with id: {} is not the next block after the latest: {}",
//             block.header.id,
//             pblock.header.id
//         );
//         return false;
//     } else if hex::encode(shash(serde_json::to_string(block).unwrap()) != serde_json::to_string(block.content.reference).unwrap()
//     {
//         log::warn!("block with id: {} has invalid hash", block.header.id);
//         return false;
//     }
//     true
// }
