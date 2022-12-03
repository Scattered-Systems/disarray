/*
   Appellation: handle <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{messages::Message, signals::ControlSignal};
use mio_extras::channel;

#[derive(Clone)]
pub struct ServerChannels {
    pub control: channel::Sender<ControlSignal>,
}

#[derive(Clone)]
pub struct ServerHandle {
    pub channels: ServerChannels,
}

impl ServerHandle {
    pub fn connect(&self, _addr: std::net::SocketAddr) {}

    pub fn broadcast(&self, _msg: Message) {}
}
