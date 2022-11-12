/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::{
    components::networking::Server,
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
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let cfname_pattern = format!("**/{}", crate::CONFIG_FILENAME);
        let builder = Config::builder()
            .add_source(collect_config_files(cfname_pattern.as_str(), true))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Err(_) => {
                Self {
                    mode: None,
                    name: None,
                    server: Server::new("127.0.0.1".to_string(), crate::MAINNET_DEFAULT_PORT)
                }
            },
            Ok(v) => v
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
