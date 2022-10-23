#[cfg(test)]
mod tests {
    use disarray::{
        blocks::{Block, calculate_block_hash}, validators::determine_block_validity, transactions::Transactions, BlockTz
    };

    #[test]
    fn test_block_validity() {
        let pblock = Block::new(0u64, "genesis_block".to_string(), Transactions::new());
        let nblock = Block::new(1u64, pblock.hash.clone(), Transactions::new());
        assert_eq!(determine_block_validity(&nblock, &pblock), true)
    }

    #[test]
    fn test_block_hash() {
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
