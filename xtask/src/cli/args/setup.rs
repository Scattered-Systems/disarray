/*
    Appellation: setup <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use clap::{Args, ValueEnum};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
pub enum Linux {
    #[default]
    Ubuntu = 0
}

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Setup {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool,
    #[clap(value_enum)]
    linux: Option<Linux>,
}

impl Setup {
    pub fn new(extras: bool, linux: Option<Linux>) -> Self {
        Self { extras, linux }
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
        if let Some(linux) = self.linux.clone() {
            match linux as i64 {
                0 => {
                    command("sudo", vec!["apt", "update", "-y"])?;
                    command("sudo", vec!["apt", "upgrade", "-y"])?;
                    command("sudo", vec!["apt", "install", "-y", "protobuf-compiler"])?;
                },
                _ => {
                    tracing::info!("Apologies, this linux variant isn't covered; please try again or implement the methods...");
                }
            }
        }
        
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
