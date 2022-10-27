#[cfg(test)]
mod tests {
    use disarray_core::{
        blocks::{Block, BlockClass, BlockContent, BlockHeader},
        crypto::hash::{block_hasher, generate_random_hash},
    };

    #[test]
    fn test_block_hash() {
        let header = BlockHeader::new(
            1,
            generate_random_hash(),
            String::new(),
            90890,
            generate_random_hash(),
        );
        let block = Block::new(BlockClass::default(), BlockContent::default(), header);
        let bhash = block_hasher(block);
        assert_eq!(&bhash, &bhash)
    }
}
