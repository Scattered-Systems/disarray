/*
    Appellation: position <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Position {
    pub depth: u128,
    pub pos: u128, // Number of proof-of-stake blocks
    pub pow: u128, // Number of proof-of-work blocks
}

impl Position {
    pub fn new(depth: u128, pos: u128, pow: u128) -> Self {
        Self { depth, pos, pow }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_position() {
        let a = Position::default();
        let b = Position::new(Default::default(), Default::default(), Default::default());
        assert_eq!(&a, &b)
    }
}
