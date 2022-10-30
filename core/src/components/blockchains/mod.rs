/*
   Appellation: blockchains <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{blockchain::*, pieces::*, utils::*};

pub(crate) mod blockchain;
pub(crate) mod pieces;

pub(crate) mod utils {

    // pub fn mmr_push_leaf(mmr: &mut MerkleMountainRange<Sha256, Vec<Hash>>, leaf_hash: Hash) {
    //     let mut leaf_hashes: Vec<Vec<u8>> = mmr
    //         .get_leaf_hashes(0, mmr.get_leaf_count().unwrap() + 1)
    //         .unwrap()
    //         .clone();
    //     leaf_hashes.push(leaf_hash);
    //     mmr.assign(leaf_hashes).unwrap();
    // }
}
