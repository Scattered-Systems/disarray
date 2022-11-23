/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::blockchains::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MinerContext {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

#[derive(Default)]
pub struct Miner {
    pub ctx: MinerContext,
}

impl Miner {
    pub fn new(ctx: MinerContext) -> Self {
        Self { ctx }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_miner() {
        let a = Miner::default();
        assert!(true)
    }
}
