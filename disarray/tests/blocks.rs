#[cfg(test)]
mod tests {
    use disarray::blocks::*;
    use scsys::prelude::{generate_random_hash, hasher, H256};

    #[test]
    fn test_block_gen_random() {
        let a = generate_random_block(BlockType::PoS, false);
        let b = generate_random_block(BlockType::PoW, false);
        assert_eq!(&a, &a);
        assert_ne!(&a, &b)
    }

    #[test]
    fn test_block_hash() {
        let block = generate_random_pos_block(true);
        let bhash: H256 = hasher(&block).as_slice().to_owned().into();
        assert_ne!(bhash, generate_random_hash())
    }
}
