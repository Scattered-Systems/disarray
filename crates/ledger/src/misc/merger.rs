/*
    Appellation: merger <blockchain>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use ckb_merkle_mountain_range::Merge;
use decanter::prelude::{Hash, Hashable, H256};
use scsys::SerdeDisplay;
use serde::{Deserialize, Serialize};

/// A simple mechanism for merging hashes for compatability with ckb-merkle-mountian-range
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, Serialize, SerdeDisplay, PartialOrd)]
pub struct Merger;

impl Merge for Merger {
    type Item = H256;

    fn merge(
        left: &Self::Item,
        right: &Self::Item,
    ) -> ckb_merkle_mountain_range::Result<Self::Item> {
        let lhs = left.0;
        let rhs = right.0;
        let mut hasher = blake3::Hasher::default();
        hasher.update(&lhs);
        hasher.update(&rhs);
        let tmp = hasher.finalize();
        let res = tmp.as_bytes();
        Ok(res.into())
    }
}
