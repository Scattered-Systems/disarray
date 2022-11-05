#[cfg(test)]
mod tests {
    use disarray_core::{
        blockchains::{Blockchain, ChainWrapperExt},
        blocks::{Block, CoreBlockWrapperExt},
    };
    use scsys::core::Timestamp;

    #[test]
    fn test_blockchain_genesis() {
        let timestamp = Timestamp::timestamp();
        let a = Blockchain::genesis(Block::genesis, timestamp);

        assert!(a.contains_hash(a.chain.keys().last().unwrap()));
    }
}
