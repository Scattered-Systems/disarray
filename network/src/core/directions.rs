/*
    Appellation: misc <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum Direction {
    Incoming,
    Outgoing,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Incoming
    }
}
