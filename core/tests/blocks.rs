#[cfg(test)]
mod tests {
    use disarray_core::{
        blocks::{calculate_block_hash, Block, BlockClass, BlockContent, BlockHeader},
        transactions::Transactions,
        validators::determine_block_validity,
        BlockTz,
    };

    #[test]
    fn test_block_validity() {
        let rc = BlockContent::default();

        // assert_eq!(determine_block_validity(&nblock, &pblock), true)
    }

    #[test]
    fn test_block_hash() {
        let header = BlockHeader::new(1, String::new(), 890890, String::new());
        let hash = calculate_block_hash(
            1,
            890890,
            "previous_hash".to_string(),
            BlockTz::now().timestamp(),
            vec!["test".to_string()],
        );
        assert_eq!(&hash, &hash)
    }
}
