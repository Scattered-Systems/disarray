/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{args::*, commands::*, context::*};

pub(crate) mod args;
pub(crate) mod commands;

use std::{sync::Arc, thread::JoinHandle};

///
pub fn new() -> CommandLineInterface {
    CommandLineInterface::default()
}
///
pub fn handle() -> JoinHandle<Arc<CommandLineInterface>> {
    let tmp = Arc::new(new());
    std::thread::spawn(move || {
        tmp.handler().expect("");
        tmp
    })
}

pub(crate) mod context {
    use super::Commands;
    use clap::Parser;

    #[derive(Clone, Debug, Hash, Parser, PartialEq)]
    #[clap(about, author, version)]
    #[clap(long_about = None)]
    pub struct CommandLineInterface {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl CommandLineInterface {
        pub fn new() -> Self {
            Self::parse()
        }
        pub fn handler(&self) -> scsys::BoxResult<&Self> {
            if self.debug {}
            if let Some(cmds) = &self.command {
                cmds.handler()?;
            }
            Ok(self)
        }
    }

    impl Default for CommandLineInterface {
        fn default() -> Self {
            Self::parse()
        }
    }
}
