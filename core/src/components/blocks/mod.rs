/*
   Appellation: blocks <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{block::Block, utils::*};

mod block;

pub(crate) mod utils {
    use crate::{BlockId, BlockHs, BlockNc, BlockTs};
    use serde_json::json;
    use sha2::Digest;

    pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res.into_bytes()
    }
    
    pub fn calculate_block_hash<Dt: Clone + serde::Serialize>(
        id: BlockId,
        nonce: BlockNc,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<Dt>,
    ) -> Vec<u8> {
        let cache = json!(
            {
                "id": id,
                "nonce": nonce,
                "previous": previous,
                "timestamp": timestamp,
                "transactions": transactions.clone()
            }
        );
        let mut hasher = sha2::Sha256::new();
        hasher.update(cache.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }
    
}