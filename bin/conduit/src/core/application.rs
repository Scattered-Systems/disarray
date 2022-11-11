/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{contexts::Context, rpc::RPCBackend, sessions::Session, states::{Stateful, States}};
use scsys::prelude::BoxResult;
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
    ///
    pub fn with_tracing(&self) -> BoxResult<&Self> {
        let name = self.ctx.settings.clone().name.unwrap_or_default();
        crate::rpc::init_tracing(&name)?;
        tracing::info!("Successfully initiated the tracing protocol...");
        Ok(self)
    }
    ///
    pub fn set_state(&mut self, state: States<T>) -> &Self {
        self.state = state;
        self
    }
    ///
    pub fn setup_backend(&self) -> RPCBackend {
        RPCBackend::new(self.ctx.settings.server.clone())
    }
    ///
    pub async fn run(&self) -> BoxResult<&Self> {
        scsys::components::logging::logger_from_env(Some("info"));

        let mut backend = self.setup_backend();
        backend.spawn().await?;

        Ok(self)
    }
}

impl<T: Stateful> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
