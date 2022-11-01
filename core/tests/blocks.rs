#[cfg(test)]
mod tests {
    use disarray_core::blocks::*;
    use scsys::prelude::{generate_random_hash, H256};

    #[test]
    fn test_block_gen_random() {
        let a = generate_random_block();
        let b = generate_random_block();
        assert_ne!(&a, &b)
    }

    #[test]
    fn test_block_hash() {
        let block = generate_random_block();
        let bhash: H256 = blake3::hash(&block.to_string().as_bytes())
            .as_bytes()
            .to_owned()
            .into();
        assert_ne!(bhash, generate_random_hash())
    }
}
