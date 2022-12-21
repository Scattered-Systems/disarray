#[cfg(test)]
mod tests {
    use disarray_sdk::ledger::{blockchains::*, blocks::*};
    use scsys::prelude::{generate_random_hash, hasher, Timestamp, H256};

    #[test]
    fn test_block_gen_random() {
        let a = generate_random_block(BlockType::PoS, false);
        let b = generate_random_block(BlockType::PoW, false);
        assert_eq!(&a, &a);
        assert_ne!(&a, &b)
    }

    #[test]
    fn test_block_hash() {
        let block = generate_random_pos_block(true);
        let bhash: H256 = hasher(&block).as_slice().to_owned().into();
        assert_ne!(bhash, generate_random_hash())
    }

    #[test]
    fn test_blockchain_genesis() {
        let timestamp = Timestamp::default().into();
        let a = Blockchain::genesis(Block::genesis, timestamp);
        let b = Blockchain::from(timestamp);
        assert!(a.contains_hash(a.chain.keys().last().unwrap()));
        assert!(b.contains_hash(b.chain.keys().last().unwrap()));
        assert!(a.contains_hash(b.chain.keys().last().unwrap()));
        assert!(b.contains_hash(a.chain.keys().last().unwrap()));
    }

    #[test]
    fn test_blockchain_insert() {
        let timestamp = Timestamp::default().into();
        let mut bc = Blockchain::genesis(Block::genesis, timestamp);
        let a = generate_random_block(BlockType::PoW, false);
        bc.insert_pow(&a);
    }
}
