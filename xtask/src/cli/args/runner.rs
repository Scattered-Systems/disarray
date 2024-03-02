/*
    Appellation: runner <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use clap::Args;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Runner {
    #[clap(long, short, value_parser)]
    package: Option<String>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    release: bool,
}

impl Runner {
    pub fn new(package: Option<String>, release: bool) -> Self {
        Self { package, release }
    }
    fn commands(&self) -> BoxResult<&Self> {
        let mut args = vec!["run"];
        if self.release {
            args.push("--release");
        }
        args.push("--");
        args.push("--h");
        command("cargo", args.clone())?;
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Initializing the application...");
        self.commands()?;
        Ok(self)
    }
}
