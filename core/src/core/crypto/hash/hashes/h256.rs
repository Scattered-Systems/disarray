/*
    Appellation: h256 <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Formal implementation of a hash, with 64 hexadecimals and compose of a generic array sized 32
*/
use crate::crypto::hash::{hasher, Hashable};
use scsys::prelude::ring;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct H256(pub [u8; 32]);

impl Hashable<H256> for H256 {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl std::fmt::Display for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 64 {
                0
            } else {
                32 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..32 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[30], &self.0[31]
        )
    }
}

impl std::convert::AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::From<&[u8; 32]> for H256 {
    fn from(input: &[u8; 32]) -> H256 {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(input);
        H256(buffer)
    }
}

impl std::convert::From<&H256> for [u8; 32] {
    fn from(input: &H256) -> [u8; 32] {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(&input.0);
        buffer
    }
}

impl std::convert::From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256(input)
    }
}

impl std::convert::From<H256> for [u8; 32] {
    fn from(input: H256) -> [u8; 32] {
        input.0
    }
}

impl std::convert::From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256(raw_hash)
    }
}

impl std::convert::From<ring::digest::Digest> for H256 {
    fn from(input: ring::digest::Digest) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256(raw_hash)
    }
}

impl Ord for H256 {
    fn cmp(&self, other: &H256) -> std::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self.0[0..16].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self.0[16..32].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other.0[0..16].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other.0[16..32].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            std::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for H256 {
    fn partial_cmp(&self, other: &H256) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
