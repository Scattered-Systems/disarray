/*
    Appellation: setup <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use clap::Args;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Setup {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool,
}

impl Setup {
    pub fn new(extras: bool) -> Self {
        Self { extras }
    }
    fn setup_artifacts(&self) -> BoxResult<&Self> {
        if std::fs::create_dir_all(&dist_dir()).is_err() {
            tracing::info!("Clearing out the previous build");
            std::fs::remove_dir_all(&dist_dir())?;
            std::fs::create_dir_all(&dist_dir())?;
        };
        Ok(self)
    }
    fn commands(&self) -> BoxResult<&Self> {
        command("rustup", vec!["default", "nightly"])?;
        command(
            "rustup",
            vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "wasm32-wasi",
                "--toolchain",
                "nightly",
            ],
        )?;
        if self.extras {
            command(
                "rustup",
                vec![
                    "component",
                    "add",
                    "clippy",
                    "rustfmt",
                    "--toolchain",
                    "nightly",
                ],
            )?;
            command("npm", vec!["install", "-g", "wasm-pack"])?;
        };
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Setting up the workspace...");
        self.setup_artifacts()?.commands()?;
        Ok(self)
    }
}
