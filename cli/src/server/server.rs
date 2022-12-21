/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use hyper::server::{conn::AddrIncoming, Builder};


use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ServerParams {
    pub address: SocketAddr,
}

impl ServerParams {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AppServer {
    pub params: ServerParams,
}

impl AppServer {
    pub fn new(params: ServerParams) -> Self {
        Self { params }
    }
    pub fn address(&self) -> &SocketAddr {
        &self.params.address
    }
    pub fn builder(&self) -> Builder<AddrIncoming> {
        tracing::debug!("Initializing the server");
        hyper::Server::bind(self.address())
    }
}
