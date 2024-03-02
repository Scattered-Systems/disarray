/*
    Appellation: args <cli>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{auto::*, builder::*, runner::*, setup::*};

pub(crate) mod builder;
pub(crate) mod runner;
pub(crate) mod setup;

pub(crate) mod auto {
    use crate::command;
    use clap::Args;
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};

    #[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Auto {}

    impl Auto {
        fn testing(&self) -> BoxResult<&Self> {
            tracing::info!("Testing the workspace...");

            command(
                "cargo",
                vec!["test", "--all", "--all-features", "--release"],
            )?;
            Ok(self)
        }
        pub fn handler(&self) -> BoxResult<&Self> {
            tracing::info!("Initializing the CI/CD pipeline");
            tracing::info!("Formatting the codespace...");
            command("cargo", vec!["fmt", "--all"])?;
            tracing::info!("Analyzing the codespace...");
            command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
            super::Builder::default().handler()?;
            self.testing()?;
            Ok(self)
        }
    }
}
