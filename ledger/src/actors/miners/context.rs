/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{
    blockchains::*, 
    handles::server::ServerHandle,
    miners::Pools,
    states::State, ControlChannel, Lock, OperatingModes, 
};
use std::convert::From;

use super::Channels;

#[derive(Clone,)]
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

impl Default for MinerContext {
    fn default() -> Self {
        let (_, r) = crossbeam::channel::unbounded();
        let chain = Blockchain::default();
        let cc: ControlChannel = r;
        let mode = OperatingModes::default();
        let pools = Pools::default();
        let server = ServerHandle::default();
        let state = State::default();
       Self::new(Lock::new(chain), cc, mode, pools, server, &state)
    }
}

impl PartialEq for MinerContext {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
    }
}

impl From<&MinerContext> for MinerContext {
    fn from(data: &MinerContext) -> Self {
        Self { 
            blockchain: data.blockchain.clone(), 
            channels: data.channels.clone(), 
            mode: data.mode.clone(), 
            pools: data.pools.clone(), 
            server: data.server.clone(), 
            state: data.state.clone()
        }
    }
}

impl From<Blockchain> for MinerContext {
    fn from(data: Blockchain) -> Self {
        let (_, r) = crossbeam::channel::unbounded();
        let cc: ControlChannel = r;
        let mode = OperatingModes::default();
        let pools = Pools::default();
        let server = ServerHandle::default();
        let state = State::default();
        Self::new(Lock::new(data), cc, mode, pools, server, &state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_miner() {
        let chain = Blockchain::default();

        let a = MinerContext::from(chain);
        let b = MinerContext::from(&a);

        assert!(&a == &b);
    }
}
