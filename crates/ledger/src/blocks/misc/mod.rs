/*
    Appellation: attr <blocks>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: attributes for blocks in the Disarray mainnet
*/

pub use self::{classification::*, difficulty::*, interface::*, justification::*, utils::*};

pub(crate) mod classification;
pub(crate) mod difficulty;
pub(crate) mod interface;
pub(crate) mod justification;

pub(crate) mod utils {}
