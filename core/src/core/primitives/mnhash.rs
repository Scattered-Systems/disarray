/*
    Appellation: mnhash <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: Short for Merge Number Hash
*/
use blake3::Hasher;
use bytes::Bytes;
use ckb_merkle_mountain_range::{Error, Merge};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct NumberHash(pub Bytes);

impl std::convert::From<u32> for NumberHash {
    fn from(num: u32) -> Self {
        let hash = blake3::hash(&num.to_le_bytes());
        NumberHash(Bytes::copy_from_slice(hash.as_bytes()))
    }
}

struct MergeNumberHash;

impl Merge for MergeNumberHash {
    type Item = NumberHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Result<Self::Item, Error> {
        let mut hasher = Hasher::new();
        hasher.update(&lhs.0);
        hasher.update(&rhs.0);
        let res = hasher.finalize();
        Ok(NumberHash(Bytes::copy_from_slice(res.as_bytes())))
    }
}
