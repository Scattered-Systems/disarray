/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::{prelude::*, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Miner {
    pub address: String,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

impl std::fmt::Display for Miner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_miner() {
        let a = Miner::default();
        let b = Miner::new(Default::default());
        assert_eq!(&a, &b)
    }
}
