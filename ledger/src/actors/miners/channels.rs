/*
   Appellation: channels <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::SignalPack;
use crate::{ContextUpdateSignal, ControlChannel};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channels() {
        let (s, r) = crossbeam::channel::unbounded();

        let a = Channels::from(r);
        assert!(true)
    }
}
