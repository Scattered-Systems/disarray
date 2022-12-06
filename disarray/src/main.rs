/*
    Appellation: Disarray <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Disarray is a dynamic blockchain network powered by a hybrid consensus protocol and engineered for tomorrow with several post-quantum considerations
*/
#[doc(inline)]
pub use self::settings::*;

pub mod agents;
pub mod cli;
pub mod contexts;
pub mod rpc;
pub mod sessions;
pub mod states;

pub(crate) mod settings;

use crate::{contexts::Context, rpc::RPCBackend, sessions::Session, states::States};
use scsys::prelude::{BoxResult, Stateful};
use std::sync::Arc;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> BoxResult {
    let app = Disarray::<contexts::Context>::default();
    println!("{}", &app);
    app.with_tracing()?.run().await?;

    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Disarray<T: Stateful> {
    pub ctx: Context,
    pub session: Session,
    pub state: Arc<States<T>>,
}

impl<T: Stateful> Disarray<T> {
    pub fn new(ctx: Context) -> Self {
        let session = Session::default();
        let state = Default::default();
        Self {
            ctx,
            session,
            state,
        }
    }
    pub async fn graceful_shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to terminate the runtime...");
        tracing::info!("Terminating the application and connected services...");
    }
    /// Creates a service handle for toggling the tracing systems implemented
    pub fn with_tracing(&self) -> BoxResult<&Self> {
        // TODO: Introduce a more refined system of tracing logged events
        self.ctx.settings.clone().logger.setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Successfully initiated the tracing protocol...");
        Ok(self)
    }
    /// Change the state of the application to the valid parameter
    pub fn set_state(&mut self, state: States<T>) -> &Self {
        self.state = Arc::new(state);
        self
    }
    /// Initialize a new backend contexutalized with the proper information
    pub fn setup_backend(&self) -> RPCBackend {
        RPCBackend::new(self.ctx.settings.server.clone())
    }
    /// A simple runner for the application, initializing a host of included systems
    pub async fn run(&self) -> BoxResult<&Self> {
        // Create an instance of the backend
        let mut backend = self.setup_backend();
        // Spawn the rpc services
        backend.spawn().await?;
        // On exit return self, owned and wrapped with the required tags
        Ok(self)
    }
}

impl<T: Stateful> std::fmt::Display for Disarray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = serde_json::json!({
            "ctx": self.ctx,
            "session": self.session
        });
        write!(f, "{}", serde_json::to_string(&data).unwrap())
    }
}
