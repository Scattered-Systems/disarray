/*
   Appellation: context <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::settings::RPCSettings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RPCContext {
    pub settings: RPCSettings
}

impl RPCContext {
    pub fn new(settings: RPCSettings) -> Self {
        Self { settings }
    }
}

impl Default for RPCContext {
    fn default() -> Self {
        Self::new(RPCSettings::default())
    }
}