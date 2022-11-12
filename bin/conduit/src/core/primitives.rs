/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {

    pub const CONFIG_FILENAME: &str = "Conduit.toml";
    pub const MAINNET_DEFAULT_PORT: u16 = 9090;
}

pub(crate) mod types {}
