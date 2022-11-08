#[cfg(test)]
mod tests {
    use disarray::{
        blockchains::{Blockchain, ChainWrapper, ChainWrapperExt},
        blocks::{generate_random_block, Block, BlockType, CoreBlockWrapperExt, CoreBlockSpec, BlockHeaderSpec},
    };
    use scsys::core::Timestamp;

    #[test]
    fn test_blockchain_genesis() {
        let timestamp = Timestamp::timestamp();
        let a = Blockchain::genesis(Block::genesis, timestamp);
        let b = Blockchain::new(timestamp);
        assert!(a.contains_hash(a.chain.keys().last().unwrap()));
        assert!(b.contains_hash(b.chain.keys().last().unwrap()));
        assert!(a.contains_hash(b.chain.keys().last().unwrap()));
        assert!(b.contains_hash(a.chain.keys().last().unwrap()));
    }

    #[test]
    fn test_blockchain_insert() {
        let timestamp = Timestamp::timestamp();
        let mut bc = Blockchain::genesis(Block::genesis, timestamp);
        let a = generate_random_block(BlockType::PoW, false);
        assert!(bc.insert_pow(&a));
    }
}
