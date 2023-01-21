/*
    Appellation: mnhash <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: Short for Merge Number Hash
*/
use blake3::{hash, Hasher};
use bytes::Bytes;
use ckb_merkle_mountain_range::{Error, Merge};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumberHash(Bytes);

impl NumberHash {
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }
}

impl std::convert::From<u32> for NumberHash {
    fn from(num: u32) -> Self {
        let hash = hash(&num.to_le_bytes());
        NumberHash(Bytes::copy_from_slice(hash.as_bytes()))
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MergeNumberHash;

impl Merge for MergeNumberHash {
    type Item = NumberHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Result<Self::Item, Error> {
        // Initialize the hasher
        let mut hasher = Hasher::new();
        // Hash the left-hand side
        hasher.update(&lhs.bytes());
        // Update the hasher by including the right-hand side
        hasher.update(&rhs.bytes());
        // Finalize the hash
        let res = hasher.finalize();
        Ok(NumberHash(Bytes::copy_from_slice(res.as_bytes())))
    }
}
