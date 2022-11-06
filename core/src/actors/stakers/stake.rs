/*
   Appellation: stake <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Stake {
    pub amount: usize,
}

impl Stake {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }
}

impl std::fmt::Display for Stake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_stake() {
        let a = Stake::default();
        let b = Stake::new(Default::default());
        assert_eq!(&a, &b)
    }
}
