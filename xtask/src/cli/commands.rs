/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Auto, Builder, Runner, Setup};
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Auto(Auto),
    Build(Builder),
    Run(Runner),
    Setup(Setup),
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Auto(auto) => {
                auto.handler()?;
            }
            Self::Build(build) => {
                tracing::info!("Compiling the codebase...");
                build.handler()?;
            }
            Self::Run(runner) => {
                runner.handler()?;
            }
            Self::Setup(setup) => {
                tracing::info!("Setting up the environment...");
                setup.handler()?;
            }
        };
        Ok(self)
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self::Setup(Default::default())
    }
}
