/*
   Appellation: headers <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{header::BlockHeader, utils::*};

pub(crate) mod header;

pub(crate) mod utils {
    use super::header::BlockHeader;
    use crate::transactions::{generate_random_signed_transaction, SignedTransaction};
    use algae::merkle::{MerkleTree, MerkleTreeWrapper};
    use scsys::{
        core::Timestamp,
        prelude::{
            generate_random_hash,
            rand::{self, Rng},
            H256,
        },
    };

    pub fn generate_random_block_header(transactions: Vec<SignedTransaction>) -> BlockHeader {
        let mut rng = rand::thread_rng();
        BlockHeader::new(
            MerkleTree::create(&transactions).root(),
            rng.gen(),
            generate_random_hash(),
            generate_random_hash(),
            generate_random_hash(),
            rng.gen(),
            Timestamp::timestamp(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}
