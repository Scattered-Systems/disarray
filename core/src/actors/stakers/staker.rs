/*
   Appellation: staker <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::Stake;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Staker {
    pub stake: Stake,
}

impl Staker {
    pub fn new(stake: Stake) -> Self {
        Self { stake }
    }
}

impl std::fmt::Display for Staker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_staker() {
        let a = Staker::default();
        let b = Staker::new(Default::default());
        assert_eq!(&a, &b)
    }
}
