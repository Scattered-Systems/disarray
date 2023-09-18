/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::ControlChannel;

use super::MinerContext;

pub struct Handle {
    pub control: ControlChannel,
}

pub struct Miner {
    pub ctx: MinerContext,
}

impl Miner {
    pub fn new(ctx: MinerContext) -> Self {
        Self { ctx }
    }
    pub fn valid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::servers::ServerHandle;
    use crate::miners::Pools;
    use crate::{blockchains::*, states::State, Lock};

    #[test]
    fn test_default() {
        let (_, r) = crossbeam::channel::unbounded();
        let chain = Blockchain::default();
        let cc: ControlChannel = r;
        let mode = Default::default();
        let pools = Pools::default();
        let server = ServerHandle::default();
        let state = State::default();
        let ctx = MinerContext::new(Lock::new(chain), cc, mode, pools, server, &state);
        let miner = Miner::new(ctx);
        assert!(miner.valid())
    }
}