/*
    Appellation: builder <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use clap::Args;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Builder {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    release: bool,
    #[arg(action = clap::ArgAction::SetFalse, long, short)]
    workspace: bool,
}

impl Builder {
    pub fn new(release: bool, workspace: bool) -> Self {
        Self { release, workspace }
    }
    fn commands(&self) -> BoxResult<&Self> {
        let mut args = vec!["build"];
        
        if self.release {
            args.push("--release");
        }
        if self.workspace {
            args.push("--workspace");
        }
        command("cargo", args)?;
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Building the workspace...");
        self.commands()?;
        Ok(self)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(false, true)
    }
}