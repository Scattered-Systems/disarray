/*
   Appellation: proof <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::path::proof_path;
use crate::merkle::MerkleDimension;
use scsys::prelude::H256;

// Returns the proof for the given index
pub fn merkle_proof(dim: MerkleDimension, nodes: Vec<H256>, index: usize) -> Vec<H256> {
    let mut proof: Vec<H256> = Vec::new();
    let mut offset: usize = 0;
    let mut leaf_size = dim.leafs;

    let proof_index = proof_path(index, leaf_size);

    for i in 0..dim.depth - 1 {
        proof.push(nodes[offset + proof_index[i]]);
        if leaf_size % 2 != 0 {
            leaf_size += 1;
        }
        offset += leaf_size;
        leaf_size /= 2;
    }
    proof
}
