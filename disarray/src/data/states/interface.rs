/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::BlockState;

pub trait Stateful {
    fn state(&self) -> &Self {
        self
    }
}

pub trait BlockchainStateWrapper {
    fn state_per_block(&self) -> BlockState;
}
