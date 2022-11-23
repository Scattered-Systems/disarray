/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::handles::server::ServerHandle;
use crate::states::State;
use crate::{
    blockchains::*, ContextUpdateSignal, ControlChannel, Lock, OperatingModes, SignedTransactions,
};
use scsys::prelude::H256;

use super::Channels;

pub struct Pools {
    pub mempool: Lock<SignedTransactions>,
    pub transpool: Lock<Vec<H256>>,
}

impl Pools {
    pub fn new(mempool: Lock<SignedTransactions>, transpool: Lock<Vec<H256>>) -> Self {
        Self { mempool, transpool }
    }
}

impl Default for Pools {
    fn default() -> Self {
        Self::new(Lock::new(Default::default()), Lock::new(Default::default()))
    }
}

pub struct MinerContext {
    pub blockchain: Lock<Blockchain>,
    pub channels: Channels,
    pub mode: OperatingModes,
    pub pools: Pools,
    pub server: ServerHandle,
    pub state: Lock<State>,
}

impl MinerContext {
    pub fn new(
        blockchain: Lock<Blockchain>,
        control: ControlChannel,
        mode: OperatingModes,
        pools: Pools,
        server: ServerHandle,
        state: &State,
    ) -> Self {
        let channels = Channels::from(control);
        let state = Lock::from(state);
        Self {
            blockchain,
            channels,
            mode,
            pools,
            server,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchains::*;

    #[test]
    fn test_default_miner() {
        let (s, r) = crossbeam::channel::unbounded();
        let chain = Blockchain::default();
        let cc: ControlChannel = r;
        let mode = Default::default();
        let pools = Pools::default();
        let server = ServerHandle::default();
        let state = State::default();
        let ctx = MinerContext::new(Lock::new(chain), cc, mode, pools, server, &state);
        assert!(true)
    }
}
