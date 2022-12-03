/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{convert::*, misc::*};

pub(crate) mod convert;

pub(crate) mod misc {
    use scsys::prelude::H256;

    /// A function wrapper converting the given vector of elements type u8
    pub fn compute_key_hash(key: Vec<u8>) -> H256 {
        key.into()
    }
}
