/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{
    cli::{cmd::Commands, CommandLineInterface},
    Context, Settings,
};
use acme::prelude::{AsyncSpawnable, Session};
use clap::Parser;
use scsys::prelude::{AsyncResult, Contextual};
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn handle() -> JoinHandle<AsyncResult> {
    tokio::spawn(async move { Ok(()) })
}

#[derive(Clone, Debug)]
pub struct Runtime {
    pub cli: Arc<CommandLineInterface>,
    pub ctx: Arc<Context>,
    pub session: Session,
}

impl Runtime {
    pub fn new(ctx: Arc<Context>) -> Self {
        let cli = Arc::new(CommandLineInterface::parse());
        let session = Session::default();
        Self { cli, ctx, session }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        let cli = self.cli.clone();

        if let Some(cmd) = cli.command() {
            match cmd {
                Commands::System(sys) => {
                    if sys.up {
                        tracing::info!("Message Recieved: System initializing...");
                    }
                }
            }
        }

        Ok(self)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(Arc::new(Context::default()))
    }
}

impl Contextual for Runtime {
    type Cnf = Settings;

    type Ctx = Context;

    fn context(&self) -> &Self::Ctx {
        self.ctx.as_ref()
    }
}

impl From<Arc<Context>> for Runtime {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for Runtime {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!({"ctx": self.ctx.as_ref()}))
    }
}

pub trait RuntimeCliSpec {
    type Cmd: clap::Subcommand;
    type Cli: clap::Parser;

    fn command(&self) -> Option<Self::Cmd>;
}
