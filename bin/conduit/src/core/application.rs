/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{contexts::Context, rpc::RPCBackend, sessions::Session, states::{Stateful, States}};
use scsys::{components::logging::logger_from_env, prelude::BoxResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Application<T: Stateful> {
    pub ctx: Context,
    pub session: Session,
    pub state: States<T>,
}

impl<T: Stateful> Application<T> {
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
        // let name = self.ctx.settings.clone().name.unwrap_or_default();
        // crate::rpc::init_tracing(&name)?;
        // Initialize the logging systems

        logger_from_env(Some("info"));
        tracing::info!("Successfully initiated the tracing protocol...");
        Ok(self)
    }
    /// Change the state of the application to the valid parameter
    pub fn set_state(&mut self, state: States<T>) -> &Self {
        self.state = state;
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

impl<T: Stateful> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
