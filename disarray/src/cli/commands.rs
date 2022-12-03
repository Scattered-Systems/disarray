/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Connect {
        #[clap(long, short, value_parser)]
        address: String,
    },
    System {
        #[arg(action = clap::ArgAction::Count, long, short)]
        on: u8,
    },
}

impl Commands {
    pub async fn handler(&self) -> &Self {
        tracing::info!("Processing commands issued to the cli...");

        self
    }
}
