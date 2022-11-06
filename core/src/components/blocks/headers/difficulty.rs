/*
   Appellation: difficulty <header>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::blocks::Resistable;
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockDifficulty {
    pub pos: H256,
    pub pow: H256,
}

impl BlockDifficulty {
    pub fn new(pos: H256, pow: H256) -> Self {
        Self { pos, pow }
    }
}

impl Hashable for BlockDifficulty {
    fn hash(&self) -> H256 {
        hasher(self).as_slice().to_owned().into()
    }
}

impl Resistable for BlockDifficulty {
    fn pos_difficulty(&self) -> H256 {
        self.pos
    }
    fn pow_difficulty(&self) -> H256 {
        self.pow
    }
}

impl Default for BlockDifficulty {
    fn default() -> Self {
        let pos = <H256>::from(crate::INITIAL_POS_DIFFICULTY);
        let pow = <H256>::from(crate::INITIAL_POW_DIFFICULTY);
        Self::new(pos, pow)
    }
}
impl std::fmt::Display for BlockDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.pos, self.pow)
    }
}

impl std::convert::From<(H256, H256)> for BlockDifficulty {
    fn from(data: (H256, H256)) -> Self {
        Self::new(data.0, data.1)
    }
}

impl std::convert::From<(&H256, &H256)> for BlockDifficulty {
    fn from(data: (&H256, &H256)) -> Self {
        Self::new(*data.0, *data.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_difficulty() {
        let a = BlockDifficulty::default();
        let b = BlockDifficulty::new(
            <H256>::from(crate::INITIAL_POS_DIFFICULTY),
            <H256>::from(crate::INITIAL_POW_DIFFICULTY),
        );
        assert_eq!(&a, &b)
    }
}
