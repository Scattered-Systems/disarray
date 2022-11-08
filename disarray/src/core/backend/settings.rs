/*
   Appellation: settings <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use scsys::{
    actors::AppConfig,
    components::{logging::Logger, networking::Server},
    core::{collect_config_files, ConfigResult},
    prelude::config::{Config, Environment},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RPCSettings {
    pub logger: Logger,
    pub server: Server,
}

impl AppConfig<'_> for RPCSettings {}

impl RPCSettings {
    pub fn new() -> ConfigResult<Self> {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/default.config.*", true));
        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for RPCSettings {
    fn default() -> Self {
        let settings = Self {
            logger: Logger::new("info".to_string()),
            server: Server::default(),
        };
        match settings.build(Some("__"), Some("**/*.config.*"), Some(true)) {
            Ok(v) => v,
            Err(e) => panic!("Configuration Error: {}", e),
        }
    }
}

impl std::fmt::Display for RPCSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
