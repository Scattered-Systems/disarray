/*
    Appellation: ping_pong <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{agent::PingPong, state::PingPongState};

pub(crate) mod agent;

pub(crate) mod state {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub enum PingPongState {
        #[default]
        Done,
        Ping,
        Pong,
        Start,
    }
}
