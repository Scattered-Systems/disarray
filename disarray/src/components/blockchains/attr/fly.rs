/*
    Appellation: fly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{
    blockchains::{Blockchain, CoreChainSpec, ChainWrapperExt},
    blocks::BlockHeader,
};
use serde::{Deserialize, Serialize};

// FlyClientProposal is a proposal sent from the prover,
// it contains current chain depth and last block header.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct FlyClientProposal {
    pub chain_depth: usize,
    pub header: BlockHeader,
}

impl FlyClientProposal {
    pub fn new(blockchain: &Blockchain) -> Self {
        FlyClientProposal {
            chain_depth: blockchain.position.depth as usize,
            header: blockchain.find_one_block(&blockchain.tip()).unwrap().header,
        }
    }
}

// FlyClientQuery is the query sent from verifier to prover,
// it contains the chain depth of a proposal and a sample of
// blocks for proof. Note sample points are < query_depth - 1.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct FlyClientQuery {
    pub query_depth: usize,
    pub sample: Vec<usize>,
}

impl FlyClientQuery {
    pub fn new(proposal_depth: usize, sample: Vec<usize>) -> Self {
        FlyClientQuery {
            query_depth: proposal_depth,
            sample,
        }
    }
}

// The proof for a single point provided by the prover. To handle
// all the sample of a query, need a Vec<FlyClientProof>.
// #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
// pub struct FlyClientProof {
//     // leaf_idx is corresponding to a number in the query sample
//     leaf_idx: usize,
//     // block header corresponding to the leaf_idx
//     pub leaf_header: BlockHeader,
//     // merkle proof for this block
//     merkle_proof: MerkleProof,
// }

// impl FlyClientProof {
//     // query depth is from the FlyClientQuery
//     pub fn new(blockchain: &Blockchain, leaf_idx: usize, query_depth: usize) -> Self {
//         // Note get_longest_chain() include genesis block with is not included in depth.
//         let leaf_hash: H256 = blockchain.get_longest_chain()[leaf_idx + 1].hash();
//         let leaf_header = blockchain.find_one_block(&leaf_hash).unwrap().header;
//         let mmr_hash = blockchain.get_longest_chain()[query_depth - 2 + 1].hash();
//         let mmr = blockchain.get_mmr(&mmr_hash);
//         let merkle_proof = MerkleProof::for_leaf_node(&mmr, leaf_idx).unwrap();
//         FlyClientProof {
//             leaf_idx,
//             leaf_header,
//             merkle_proof,
//         }
//     }

//     // only deals with first two step verification in the paper.
//     pub fn verify(self, mmr_root: Hash) -> bool {
//         assert!(self
//             .merkle_proof
//             .verify_leaf::<Sha256>(
//                 &mmr_root[..],
//                 self.leaf_header.hash().as_ref(),
//                 self.leaf_idx
//             )
//             .is_ok());
//         true
//     }
// }
