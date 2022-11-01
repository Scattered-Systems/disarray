#[cfg(test)]
mod tests {
    use disarray_core::{blockchains::{Blockchain, ChainWrapper, ChainWrapperExt}, blocks::{generate_genesis_block, generate_random_block}};
    use scsys::core::Timestamp;

    #[test]
    fn test_blockchain_genesis() {
        let timestamp = Timestamp::timestamp();
        // let b1 = generate_pow_block();
        let mut a = Blockchain::genesis(generate_genesis_block, timestamp);
        
        let rpos = generate_random_block(&a.tip());
        
        assert!(a.contains_hash(a.chain.keys().last().unwrap()));
        assert!(a.insert_pos(&rpos, true));
        // assert!(a.contains_hash(&rpos.header.merkle_root))

    }
}
