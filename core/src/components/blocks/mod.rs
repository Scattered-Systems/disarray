/*
   Appellation: blocks <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{attr::*, block::*, utils::*};

pub(crate) mod attr;
pub(crate) mod block;

pub(crate) mod utils {
    #![allow(clippy::too_many_arguments)]
    use crate::{
        blocks::{
            Block, BlockContent, BlockDifficulty, BlockHeader, BlockJustification, BlockType,
        },
        transactions::SignedTransaction,
        BlockHs, BlockId, BlockNc, BlockTs,
    };
    use algae::merkle::{MerkleTree, MerkleTreeWrapper};
    use scsys::prelude::{hasher, H256};
    use serde_json::json;

    pub fn calculate_block_hash(
        id: BlockId,
        nonce: BlockNc,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<SignedTransaction>,
    ) -> H256 {
        let cache = json!(
            {
                "id": id,
                "nonce": nonce,
                "previous": previous,
                "timestamp": timestamp,
                "transactions": transactions
            }
        );
        hasher(&cache).as_slice().to_owned().into()
    }

    pub fn generate_pow_block(
        data: &[SignedTransaction],
        difficulty: BlockDifficulty,
        justification: BlockJustification,
        transaction_ref: &[H256],
        parent: &H256,
        nonce: u32,
        rand: u128,
        timestamp: i64,
        selfish_block: bool,
    ) -> Block {
        // let mmr_root: parent_mmr.get_merkle_root().unwrap(),
        let mt = MerkleTree::create(data);
        let block_type = BlockType::PoW;
        let content = BlockContent::new(data.to_vec(), transaction_ref.to_vec());
        let header = BlockHeader::new(
            difficulty,
            justification,
            nonce,
            *parent,
            rand,
            mt.root(),
            timestamp,
        );

        Block::new(content, header, block_type, selfish_block)
    }

    pub fn generate_genesis_block(initial_time: i64) -> Block {
        let content = BlockContent::default();
        let difficulty = BlockDifficulty::default();
        let justification = BlockJustification::default();
        let block_type = BlockType::from(true);
        let selfish_block = false;

        let header = BlockHeader::new(
            difficulty,
            justification,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            initial_time,
        );
        Block::new(content, header, block_type, selfish_block)
    }

    pub fn generate_random_block(class: BlockType, selfish: bool) -> Block {
        let content = super::generate_random_block_content();
        let header = super::generate_random_block_header(content.data.clone());

        Block::new(content, header, class, selfish)
    }

    pub fn generate_random_pos_block(selfish: bool) -> Block {
        generate_random_block(BlockType::PoS, selfish)
    }

    pub fn generate_random_pow_block(selfish: bool) -> Block {
        generate_random_block(BlockType::PoW, selfish)
    }
}
