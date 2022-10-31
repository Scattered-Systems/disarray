#[cfg(test)]
mod tests {
    use disarray_core::blockchains::*;
    use scsys::core::Timestamp;

    #[test]
    fn test_blockchain_genesis() {
        let timestamp = Timestamp::timestamp();
        let a = Blockchain::new(timestamp);
        let chain = a.chain.clone();
        assert!(a.contains_hash(chain.keys().last().unwrap()))
    }
}
