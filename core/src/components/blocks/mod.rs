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
    use super::{Block, BlockContent, BlockHeader, BlockType};
    use crate::{
        merkle::MerkleTree, transactions::SignedTransaction, BlockHs, BlockId, BlockNc, BlockTs,
    };
    use scsys::{
        crypto::hash::{generate_random_hash, hasher, H256},
        prelude::rand::{self, Rng},
    };
    use serde::Serialize;
    use serde_json::json;

    pub fn generate_pow_block(
        data: &Vec<SignedTransaction>,
        transaction_ref: &Vec<H256>,
        parent: &H256,
        nonce: u32,
        pow_difficulty: &H256,
        pos_difficulty: &H256,
        timestamp: i64,
        vrf_proof: &Vec<u8>,
        vrf_hash: &Vec<u8>,
        vrf_pub_key: &[u8],
        rand: u128,
        selfish_block: bool,
    ) -> Block {
        let mt = MerkleTree::new(data);
        let block_type = BlockType::PoW;
        let content = BlockContent::new(data.to_vec(), transaction_ref.to_vec());
        let header = BlockHeader {
            parent: *parent,
            nonce: nonce,
            pow_difficulty: *pow_difficulty,
            pos_difficulty: *pos_difficulty,
            timestamp: timestamp,
            merkle_root: mt.root(),
            //mmr_root: parent_mmr.get_merkle_root().unwrap(),
            vrf_proof: vrf_proof.to_vec(),
            vrf_hash: vrf_hash.to_vec(),
            vrf_pub_key: vrf_pub_key.to_vec(),
            rand: rand,
        };
        Block {
            header,
            content,
            block_type,
            selfish_block: selfish_block,
        }
    }

    pub fn generate_genesis_block(initial_time: i64) -> Block {
        let content = BlockContent::default();
        let block_type = BlockType::from(true);
        let selfish_block = false;
        let header = BlockHeader {
            parent: Default::default(),
            nonce: Default::default(),
            //pow_difficulty: <H256>::from([1; 32]),
            pow_difficulty: <H256>::from([
                0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]),
            pos_difficulty: <H256>::from([1; 32]),
            timestamp: initial_time,
            merkle_root: Default::default(),
            // mmr_root: MerkleMountainRange::<Sha256, Vec<Hash>>::new(Vec::new()).get_merkle_root().unwrap(),
            vrf_proof: Default::default(),
            vrf_hash: Default::default(),
            vrf_pub_key: Default::default(),
            rand: Default::default(),
        };
        Block {
            header,
            content,
            block_type,
            selfish_block,
        }
    }

    pub fn generate_random_block_content() -> BlockContent {
        BlockContent::new(Vec::new(), vec![generate_random_hash()])
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
        hasher(&cache).as_slice().to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::crypto::hash::{generate_random_hash, hasher, H256};

    #[test]
    fn test_block_default() {
        let block1 = Block::new(
            generate_random_block_content(),
            generate_random_block_header(),
            BlockType::from(false),
            false,
        );
        let block2 = Block::new(
            generate_random_block_content(),
            generate_random_block_header(),
            BlockType::from(true),
            false,
        );
        assert_ne!(block1, block2)
    }

    #[test]
    fn test_block_hash() {
        let block = Block::new(
            generate_random_block_content(),
            generate_random_block_header(),
            BlockType::from(false),
            false,
        );
        let bhash: H256 = hasher(&block).as_slice().to_owned().into();
        assert_ne!(bhash, generate_random_hash())
    }
}
