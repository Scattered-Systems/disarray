/*
    Appellation: reqres <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::handles::peers::Handle;
use crossbeam::channel as cbc;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct ConnectRequest {
    pub addr: SocketAddr,
    pub chan: cbc::Sender<std::io::Result<Handle>>,
}

impl ConnectRequest {
    pub fn new(addr: SocketAddr, chan: cbc::Sender<std::io::Result<Handle>>) -> Self {
        Self { addr, chan }
    }
}
