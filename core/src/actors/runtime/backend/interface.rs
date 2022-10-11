/*
   Appellation: rpc <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::context::RPCContext;
use scsys::core::{BoxResult, logging::logger_from_env};
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RPCBackend {
    pub ctx: RPCContext
}

impl RPCBackend {
    pub fn new(ctx: RPCContext) -> Self {
        Self { ctx }
    }
    pub fn logging(&self, mode: Option<&str>) -> &Self {
        logger_from_env(mode);
        self
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        Ok(self)
    }
}

impl Default for RPCBackend {
    fn default() -> Self {
        Self::new(RPCContext::default())
    }
}
