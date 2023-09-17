/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, ConfigResult, Configurable, SerdeDisplay};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct NetworkSettings {
    pub mode: String,
    pub port: u16,
}

impl NetworkSettings {
    pub fn new(mode: String, port: u16) -> Self {
        Self { mode, port }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .set_default("mode", "mainnet")?
            .set_default("port", 9999)?;

        if let Ok(v) = std::env::var("NETWORK_MODE") {
            builder = builder.set_override("logger.level", v)?;
        };
        if let Ok(v) = std::env::var("NETWORK_MODE") {
            builder = builder.set_override("port", v)?;
        };
        // Add in related environment variables
        builder = builder.add_source(Environment::default().separator("__").prefix("DISARRAY"));
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }
}

impl Configurable for NetworkSettings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for NetworkSettings {
    fn default() -> Self {
        let d = Self::new("mainnet".to_string(), 9999);
        Self::build().unwrap_or(d)
    }
}
