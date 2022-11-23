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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchains::*;

    #[test]
    fn test_default() {
        let (s, r) = crossbeam::channel::unbounded();
        let chain = Blockchain::default();
        let cc: ControlChannel = r;
        let mode = Default::default();
        let pools = Default::default();
        let server = Default::default();
        let ctx = MinerContext::new(Lock::new(chain), cc, mode, pools, server, state);
        assert!(true)
    }
}
