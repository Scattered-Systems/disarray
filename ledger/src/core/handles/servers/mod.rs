/*
    Appellation: server <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{controller::*, reqres::*, server::*, utils::*};

pub(crate) mod controller;
pub(crate) mod reqres;
pub(crate) mod server;
pub(crate) mod utils {}
