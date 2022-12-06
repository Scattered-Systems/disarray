/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{DEFAULT_CNF_PATTERN, DEFAULT_PORT_MAINNET, DEFAULT_SYSTEM_NAME, LOCALHOST};
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, ConfigResult, Configurable, Logger, Server};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub name: Option<String>,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("name", Some(DEFAULT_SYSTEM_NAME))?
            .set_default("logger.level", "info")?
            .set_default("server.host", LOCALHOST)?
            .set_default("server.port", DEFAULT_PORT_MAINNET)?;
        match try_collect_config_files(DEFAULT_CNF_PATTERN, false) {
            Err(_) => {}
            Ok(v) => {
                builder = builder.add_source(v);
            }
        }
        match std::env::var("RUST_LOG") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("logger.level", Some(v))?;
            }
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("server.port", v)?;
            }
        };

        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Settings {
            name: Some(DEFAULT_SYSTEM_NAME.to_string()),
            logger: Default::default(),
            server: Server::new(LOCALHOST.to_string(), DEFAULT_PORT_MAINNET),
        };
        Self::build().unwrap_or(d)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
