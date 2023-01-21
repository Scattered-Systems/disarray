/*
    Appellation: proofs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::blocks::BlockHeader;
use crate::{Blockchain, ChainWrapperExt, Merger};
use ckb_merkle_mountain_range::MerkleProof;
use decanter::prelude::{Hashable, H256};

/// Describes the payload implemented for chain proofs
pub struct ChainProofPayload {
    // leaf_idx is corresponding to a number in the query sample
    leaf_idx: usize,
    // block header corresponding to the leaf_idx
    pub leaf_header: BlockHeader,
    // merkle proof for this block
    merkle_proof: MerkleProof<H256, Merger>,
}

impl ChainProofPayload {
    // query depth is from the FlyClientQuery
    pub fn new(blockchain: &Blockchain, leaf_idx: usize, query_depth: usize) -> Self {
        // Note get_longest_chain() include genesis block with is not included in depth.
        let leaf_hash: H256 = blockchain.get_longest_chain()[leaf_idx + 1].hash();
        let leaf_header = blockchain.find_one_block(&leaf_hash).unwrap().header;
        let mmr_hash = blockchain.get_longest_chain()[query_depth - 2 + 1].hash();
        let mmr = blockchain.get_mmr(&mmr_hash);
        let merkle_proof = mmr.gen_proof(vec![0, leaf_idx as u64]).unwrap();
        Self {
            leaf_idx,
            leaf_header,
            merkle_proof,
        }
    }
    // only deals with first two step verification in the paper.
    // TODO: Devise more robust verfication steps
    pub fn verify(self, mmr_root: H256) -> bool {
        let a = vec![(self.leaf_idx as u64, self.leaf_header.hash())];
        self.merkle_proof.verify(mmr_root, a).is_ok()
    }
}
