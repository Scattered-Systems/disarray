/*
   Appellation: settings <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use scsys::{
    collect_config_files,
    prelude::{
        config::{Config, ConfigError, Environment},
        Logger, Server,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RPCSettings {
    pub logger: Logger,
    pub server: Server,
}

impl RPCSettings {
    pub fn build() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/default.config.*", true));
        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(Environment::default().separator("__"));

        builder
            .build()
            .expect("Failed to build the configuration...")
            .try_deserialize()
    }
}

impl Default for RPCSettings {
    fn default() -> Self {
        match Self::build() {
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
