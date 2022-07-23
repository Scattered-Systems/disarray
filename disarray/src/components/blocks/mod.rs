/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use block::*;
pub use utils::*;

mod block;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transaction<Id = u8, Key = String, Ts = i64, Data = String> {
    id: Id,
    key: Key,
    timestamp: Ts,
    data: Vec<Data>,
}

impl<Id, Key, Ts, Data> Transaction<Id, Key, Ts, Data> {
    pub fn constructor(id: Id, key: Key, timestamp: Ts, data: Vec<Data>) -> Self {
        Self {
            id,
            key,
            timestamp,
            data,
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction(\nid={:#?},\nkey={:#?},\ntimestamp={:#?},\ndata={:#?}\n)",
            self.id, self.key, self.timestamp, self.data
        )
    }
}

mod utils {
    use crate::{BlockData, DIFFICULTY_PREFIX};
    use scsys::{BlockHs, BlockId, BlockNc, BlockTs};
    use sha2::Digest;

    pub fn create_block_by_mining<Dt: Clone + serde::Serialize>(
        id: BlockId,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: Vec<Dt>,
    ) -> (BlockNc, BlockHs) {
        log::info!("Mining a new block...");
        let mut nonce = 0;
        loop {
            if nonce % 100000 == 0 {
                log::info!("nonce: {}", nonce);
            }
            let hash = calculate_block_hash(
                id,
                nonce,
                previous.clone(),
                timestamp.clone(),
                transactions.clone(),
            );
            let binary_hash = convert_hash_into_binary(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX.as_ref()) {
                log::info!(
                    "mined! nonce: {}, hash: {}, binary hash: {:#?}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(hash).into());
            }
            nonce += 1;
        }
    }

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
        let cache = serde_json::json!(
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_block_hash() {
            let id: BlockId = 10;
            let nonce: BlockNc = 890890;
            let previous = "previous_hash".to_string();
            let timestamp: BlockTs = scsys::BlockTz::now().timestamp();
            let transactions = vec!["test".to_string()];
            let hash =
                calculate_block_hash(id, nonce, previous.clone(), timestamp, transactions.clone());
            assert_eq!(&hash, &hash)
        }
    }
}
