/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{
    handles::servers::ServerHandle, miners::Pools, states::State, Blockchain, ControlChannel,
    OperatingModes,
};
use scsys::Locked;
use std::convert::From;

use super::Channels;

#[derive(Clone)]
pub struct MinerContext {
    pub blockchain: Locked<Blockchain>,
    pub channels: Channels,
    pub mode: OperatingModes,
    pub pools: Pools,
    pub server: ServerHandle,
    pub state: Locked<State>,
}

impl MinerContext {
    pub fn new(
        blockchain: Locked<Blockchain>,
        control: ControlChannel,
        mode: OperatingModes,
        pools: Pools,
        server: ServerHandle,
        state: &State,
    ) -> Self {
        let channels = Channels::from(control);
        let state = Locked::new(state.clone().into());
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
        Self::new(Locked::new(chain.into()), cc, mode, pools, server, &state)
    }
}

impl PartialEq for MinerContext {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
    }
}

impl From<&MinerContext> for MinerContext {
    fn from(data: &MinerContext) -> Self {
        data.clone()
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
        Self::new(Locked::new(data.into()), cc, mode, pools, server, &state)
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
