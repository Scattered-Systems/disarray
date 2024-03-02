pub use self::{content::*, header::*, utils::*};

pub(crate) mod content;
pub(crate) mod header;

pub(crate) mod utils {
    use crate::{
        blocks::{BlockContent, BlockDifficulty, BlockHeader, BlockJustification},
        transactions::{generate_random_signed_transaction, SignedTransaction},
    };
    use algae::merkle::{MerkleTree, MerkleTreeWrapper};
    use decanter::prelude::generate_random_hash;
    use rand::Rng;
    use scsys::prelude::Timestamp;

    pub fn generate_random_block_content() -> BlockContent {
        BlockContent::new(
            vec![generate_random_signed_transaction()],
            vec![generate_random_hash()],
        )
    }

    pub fn generate_random_block_header(transactions: Vec<SignedTransaction>) -> BlockHeader {
        let mut rng = rand::thread_rng();
        let difficulty = BlockDifficulty::new(generate_random_hash(), generate_random_hash());
        let justification = BlockJustification::default();
        BlockHeader::new(
            difficulty,
            justification,
            rng.gen(),
            generate_random_hash(),
            rng.gen(),
            MerkleTree::create(&transactions).root(),
            Timestamp::default().into(),
        )
    }
}
