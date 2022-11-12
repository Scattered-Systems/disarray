/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{DEFAULT_CNF_PATTERN, DEFAULT_PORT_MAINNET, DEFAULT_SYSTEM_NAME, LOCALHOST};
use scsys::{
    components::{logging::Logger, networking::Server},
    prelude::{
        collect_config_files,
        config::{Config, Environment},
        ConfigResult,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub mode: Option<String>,
    pub name: Option<String>,
    pub server: Server,
    pub tracing: Option<Logger>,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let builder = Config::builder()
            .add_source(collect_config_files(DEFAULT_CNF_PATTERN, true))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Err(e) => {
                println!("{}", e);
                Self {
                    mode: Some("production".to_string()),
                    name: Some(DEFAULT_SYSTEM_NAME.to_string()),
                    server: Server::new(LOCALHOST.to_string(), DEFAULT_PORT_MAINNET),
                    tracing: Some(Default::default()),
                }
            }
            Ok(v) => v,
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
