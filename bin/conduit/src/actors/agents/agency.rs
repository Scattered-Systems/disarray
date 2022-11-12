/*
    Appellation: agency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub enum Agency {
    #[default]
    Client,
    Server,
}

impl std::convert::From<bool> for Agency {
    fn from(data: bool) -> Self {
        if data {
            Self::Client
        } else {
            Self::Server
        }
    }
}

impl std::fmt::Display for Agency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
