/*
   Appellation: staker <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::blocks::BlockType;
use decanter::prelude::{Hash, Hashable};
use scsys::SerdeDisplay;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Validator {
    pub class: BlockType,
}

impl Validator {
    pub fn new(class: BlockType) -> Self {
        Self { class }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validator() {
        let a = Validator::default();
        let b = Validator::new(Default::default());
        assert_eq!(&a, &b)
    }
}
