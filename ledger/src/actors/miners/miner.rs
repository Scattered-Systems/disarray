/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blockchains::*, ContextUpdateSignal, ControlChannel, OperatingModes, SignedTransactions};
use crate::handles::server::ServerHandle;
use crate::states::State;
use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use scsys::prelude::H256;
use std::sync::{Arc, Mutex};

/// Bootstrap's a channel reciever and sender together under one structure
pub struct SignalPack<T = ContextUpdateSignal> {
    pub receiver: Receiver<T>,
    pub sender: Sender<T>,
}

impl<T> SignalPack<T> {
    pub fn new(receiver: Receiver<T>, sender: Sender<T>) -> Self {
        Self { receiver, sender }
    }
    pub fn unbounded() -> Self {
        let (sender, recv) = unbounded();
        Self::new(recv, sender)
    }
}

impl<T> Default for SignalPack<T> {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Handles the channels for the given context
pub struct Channels {
    pub controller: ControlChannel,
    pub update: SignalPack<ContextUpdateSignal>,
}

impl Channels {
    pub fn new(controller: ControlChannel, update: SignalPack<ContextUpdateSignal>) -> Self {
        Self { controller, update }
    }
}

impl std::convert::From<ControlChannel> for Channels {
    fn from(data: ControlChannel) -> Self {
        Self::new(data, Default::default())
    }
}

pub struct Lock<T>(pub Arc<Mutex<T>>);

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self(Arc::new(Mutex::from(data)))
    }
}

impl<T> std::convert::From<&T> for Lock<T>
where
    T: Clone,
{
    fn from(data: &T) -> Self {
        Self(Arc::new(Mutex::from(data.clone())))
    }
}

pub struct Pools {
    pub mempool: Lock<SignedTransactions>,
    pub transpool: Lock<Vec<H256>>,
}

pub struct Handle {
    pub control: ControlChannel,
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
        // let a = Miner::default();
        assert!(true)
    }
}
