/*
   Appellation: chains
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use chain::*;

mod chain;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ChainStates {
    Initializing,
    Invalid,
    Valid,
}

pub trait ChainSpec<Addr, Conf, Cont, Data> {
    fn constructor(&self, address: Addr, config: Conf, context: Cont, data: Vec<Data>) -> Self
        where
            Self: Sized;
}
