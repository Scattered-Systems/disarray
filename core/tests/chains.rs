#[cfg(test)]
mod tests {
    use disarray_core::{blocks::Block, blockchains::Blockchain, validators::determine_chain_validity};

    #[test]
    fn test_blockchain_default() {
        let a = Blockchain::default();
        let b = Blockchain::new(std::net::SocketAddr::from(([0, 0, 0, 0], 9090)));
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
