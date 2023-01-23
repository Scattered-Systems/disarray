/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::prelude::{fnl_remove, StatePack};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

pub type State = scsys::prelude::State<States>;

#[derive(
    Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    #[default]
    Valid = 1,
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}
