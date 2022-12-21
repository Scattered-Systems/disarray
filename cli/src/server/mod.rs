/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, server::*};

pub(crate) mod server;

pub(crate) mod context {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct ServerContext {}

    impl ServerContext {
        pub fn new() -> Self {
            Self {}
        }
    }
}
