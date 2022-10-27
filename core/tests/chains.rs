#[cfg(test)]
mod tests {
    use disarray_core::{
        blockchains::{BlockData, Blockchain},
        blocks::{Block, BlockClass, BlockContent, BlockHeader},
        crypto::hash::{block_hasher, generate_random_hash},
    };

    #[test]
    fn test_blockchain_default() {
        let header = BlockHeader::new(
            1,
            generate_random_hash(),
            String::new(),
            90890,
            generate_random_hash(),
        );
        let block = Block::new(BlockClass::default(), BlockContent::default(), header);
        log::info!("{:?}", block_hasher(block.clone()));
        let mut a = Blockchain::new();
        a.blockchain.push(BlockData { block, height: 0 });
        let b = Blockchain::new();
        assert_ne!(a, b)
    }
}
