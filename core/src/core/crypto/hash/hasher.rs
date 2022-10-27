/*
   Appellation: mine <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::{hashes::H256, utils::hasher};
use serde::Serialize;

pub struct Hasher<T: Clone + Serialize, H: From<Vec<u8>> = H256> {
    pub data: T,
    pub hash: H,
}

impl<T: Clone + Serialize, H: From<Vec<u8>>> Hasher<T, H> {
    pub fn new(data: T) -> Self {
        let hash: H = hasher(&data).into();
        Self { data, hash }
    }
}
