/*
    Appellation: block_fetch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{agent::*, state::*};

pub(crate) mod agent;

pub(crate) mod state {
    pub enum State {}
}
