/*
   Appellation: handle <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{signals::ControlSignal, messages::Message};
use mio_extras::channel;


#[derive(Clone)]
pub struct ServerChannels {
    pub control: channel::Sender<ControlSignal>
}

#[derive(Clone)]
pub struct ServerHandle {
    pub channels: ServerChannels,
}

impl ServerHandle {
    pub fn connect(&self, addr: std::net::SocketAddr) {
        
    }

    pub fn broadcast(&self, msg: Message) {
    }
}