/*
    Appellation: builder <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Connector {
    #[clap(long, short, value_parser)]
    address: Option<String>,
}

impl Connector {
    pub fn new(address: Option<String>) -> Self {
        Self { address }
    }
    fn commands(&self) -> BoxResult<&Self> {
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::debug!("Connector Initialized; handling inputs...");
        if let Some(addr) = self.address.clone() {
            tracing::info!("{:?}", addr);
        };
        self.commands()?;
        Ok(self)
    }
}

impl Default for Connector {
    fn default() -> Self {
        Self::new(None)
    }
}
