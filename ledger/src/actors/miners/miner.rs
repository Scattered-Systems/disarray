/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Miner {
    pub name: String
}

impl Miner {
    pub fn new(name: String) -> Self {
        Self { name }
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