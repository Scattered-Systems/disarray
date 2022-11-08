/*
    Appellation: state <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize)]
pub enum DecodeState {
    #[default]
    Length,
    Payload,
}

#[derive(Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize)]
pub enum WriteState {
    #[default]
    Length,
    Payload,
}
