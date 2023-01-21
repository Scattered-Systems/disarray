/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::BlockTs;
use decanter::prelude::{hasher, Hashable, H256};
use scsys::prelude::SerdeDisplay;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Epoch {
    pub size: u128,
    pub time: BlockTs,
}

impl Epoch {
    pub fn new(size: u128, time: BlockTs) -> Self {
        Self { size, time }
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new(400, 120_000_000)
    }
}

impl Hashable for Epoch {
    fn hash(&self) -> H256 {
        hasher(&self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_epoch() {
        let a = Epoch::default();
        let b = Epoch::new(400, 120_000_000);
        assert_eq!(&a, &b)
    }
}
