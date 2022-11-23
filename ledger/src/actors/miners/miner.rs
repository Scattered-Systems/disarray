/*
   Appellation: miner <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{blockchains::*, ControlSignal, ContextUpdateSignal, OperatingModes};
use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};

pub struct Channels {
    pub controller: Receiver<ControlSignal> 
}

impl Channels {
    pub fn new(controller: Receiver<ControlSignal>) -> Self {
        Self { controller }
    }
}

pub struct Signals<T = ContextUpdateSignal> {
    pub receiver: Receiver<T>,
    pub sender: Sender<T>
}


pub struct Controller {
    pub channel: Receiver<ControlSignal>,
    pub signals: Signals 
}


pub struct MinerContext {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub channels: Channels,
    pub state: OperatingModes
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
