#[cfg(test)]
mod tests {
    use disarray_core::{
        blockchains::Blockchain, blocks::Block, validators::determine_chain_validity,
    };

    #[test]
    fn test_blockchain_default() {
        let a = Blockchain::default();
        let b = Blockchain::from(a.clone());
        assert_eq!(a, b)
    }

    #[test]
    fn test_blockchain_update() {
        let mut blockchain = Blockchain::default().genesis();
        blockchain.add_block(Block::new(
            1u64,
            blockchain.chain.last().unwrap().hash.clone(),
            Vec::new(),
        ));
        assert!(determine_chain_validity(&blockchain.chain))
    }
}
