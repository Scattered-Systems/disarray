/*
    Appellation: engines <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{engine::*, specs::*, utils::*};

pub(crate) mod engine;

pub(crate) mod specs {
    pub trait CoreEngineSpec {}

    pub trait CoreEngineWrapper: CoreEngineSpec {}

    pub trait CoreEngineWrapperExt: CoreEngineWrapper {}
}

pub(crate) mod utils {}
