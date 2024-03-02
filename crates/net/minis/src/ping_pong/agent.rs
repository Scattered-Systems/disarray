/*
    Appellation: agent <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::PingPongState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PingPong {
    pub state: PingPongState,
}

impl PingPong {
    pub fn new(state: PingPongState) -> Self {
        Self { state }
    }
}

impl Default for PingPong {
    fn default() -> Self {
        Self::new(PingPongState::default())
    }
}
