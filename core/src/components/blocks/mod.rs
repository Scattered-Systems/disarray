/*
   Appellation: blocks <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{block::*, content::*, header::*, utils::*};

pub(crate) mod block;
pub(crate) mod content;
pub(crate) mod header;

pub(crate) mod utils {
    use super::{BlockContent, BlockHeader};
    use crate::{
        crypto::hash::{generate_random_hash, hasher, H256},
        BlockHs, BlockId, BlockNc, BlockTs,
    };
    use scsys::prelude::rand::{self, Rng};
    use serde::Serialize;
    use serde_json::json;

    pub fn generate_random_block_content() -> BlockContent {
        BlockContent::new(Vec::new(),generate_random_hash())
    }

    pub fn generate_random_block_header() -> BlockHeader {
        let mut rng = rand::thread_rng();
        BlockHeader::new(
            generate_random_hash(),
            rng.gen(),
            generate_random_hash(),
            generate_random_hash(),
            generate_random_hash(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res.into_bytes()
    }

    pub fn calculate_block_hash<Dt: Clone + Serialize>(
        id: BlockId,
        nonce: BlockNc,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<Dt>,
    ) -> H256 {
        let cache = json!(
            {
                "id": id,
                "nonce": nonce,
                "previous": previous,
                "timestamp": timestamp,
                "transactions": transactions.clone()
            }
        );
        hasher(&cache).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::{generate_random_hash, hasher, H256};

    #[test]
    fn test_block_default() {
        let block1 = Block::new(
            BlockClass::default(),
            generate_random_block_content(),
            generate_random_block_header(),
        );
        let block2 = Block::new(
            BlockClass::default(),
            generate_random_block_content(),
            generate_random_block_header(),
        );
        assert_ne!(block1, block2)
    }

    #[test]
    fn test_block_hash() {
        let block = Block::new(
            BlockClass::default(),
            generate_random_block_content(),
            generate_random_block_header(),
        );
        let bhash: H256 = hasher(&block).into();
        assert_ne!(bhash, generate_random_hash())
    }
}
