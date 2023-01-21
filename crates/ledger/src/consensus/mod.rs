/*
    Appellation: consensus <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{drivers::*, options::*, specs::*};

pub(crate) mod drivers;
pub(crate) mod options;

pub(crate) mod specs {
    pub trait Consentual: ToString {
        type Proof: ToString;

        fn agreement(&self) -> bool;
    }
}
