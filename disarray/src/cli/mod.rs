/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{commands::*, context::*};

pub(crate) mod commands;

pub(crate) mod context {
    use super::Commands;
    use clap::Parser;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = "Welcome, listed below is some availible commands for the application")]
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::Count, long, short)]
        pub debug: u8,
    }

    impl Cli {
        pub fn new() -> Self {
            Self::parse()
        }
    }

    impl Default for Cli {
        fn default() -> Self {
            Self::new()
        }
    }
}
