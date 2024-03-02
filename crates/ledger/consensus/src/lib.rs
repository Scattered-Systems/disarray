/*
   Appellation: disarray-consensus <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{drivers::*, options::*};

pub(crate) mod drivers;
pub(crate) mod options;


pub trait Consentual: ToString {
    type Proof: ToString;

    fn agreement(&self) -> bool;
}
