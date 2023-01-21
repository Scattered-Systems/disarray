/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::fnl_remove;
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
#[strum(serialize_all = "snake_case")]
pub enum Events {
    #[default]
    Idle = 0,
    Genesis = 1,

    Update = 9,
}

impl From<Events> for i64 {
    fn from(e: Events) -> i64 {
        e as i64
    }
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}
