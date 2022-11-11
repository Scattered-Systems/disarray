/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::states::{reqres::{Request, Response}, Stateful};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]

pub enum States<T: Stateful> {
    Idle,
    Req(Request<T>),
    Res(Response<T>),
}

impl<T: Stateful> States<T> {}

impl<T: Stateful> Default for States<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T: Stateful> fmt::Display for States<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
