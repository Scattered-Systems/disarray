pub use self::{content::*, header::*, utils::*};

pub(crate) mod content;
pub(crate) mod header;

pub(crate) mod utils {
    use crate::{
        blocks::{BlockContent, BlockDifficulty, BlockHeader, BlockJustification},
        transactions::{generate_random_signed_transaction, SignedTransaction},
    };
    use algae::merkle::{MerkleTree, MerkleTreeWrapper};
    use rand::Rng;
    use scsys::prelude::{generate_random_hash, Timestamp};

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
        let root = MerkleTree::create(&transactions).root();
        BlockHeader::new(
            difficulty,
            justification,
            rng.gen(),
            generate_random_hash(),
            rng.gen(),
            root,
            Timestamp::ts(),
        )
    }
}
