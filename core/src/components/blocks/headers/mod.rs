/*
   Appellation: headers <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{difficulty::*, header::BlockHeader, justification::*, utils::*};

pub(crate) mod difficulty;
pub(crate) mod header;
pub(crate) mod justification;

pub(crate) mod utils {
    use super::{BlockDifficulty, BlockHeader, BlockJustification};
    use crate::transactions::SignedTransaction;
    use algae::merkle::{MerkleTree, MerkleTreeWrapper};
    use scsys::{
        core::Timestamp,
        prelude::{
            generate_random_hash,
            rand::{self, Rng},
        },
    };

    pub fn generate_random_block_header(transactions: Vec<SignedTransaction>) -> BlockHeader {
        let mut rng = rand::thread_rng();
        let difficulty = BlockDifficulty::new(generate_random_hash(), generate_random_hash());
        let justification = BlockJustification::default();
        let root = MerkleTree::create(&transactions).root();
        BlockHeader::new(
            difficulty,
            justification,
            rng.gen(),
            generate_random_hash(),
            rng.gen(),
            root,
            Timestamp::timestamp(),
        )
    }
}
