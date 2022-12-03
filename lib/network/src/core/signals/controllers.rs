/*
   Appellation: signals <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{contexts::handlers::BaseHandle, messages::Message};

#[derive(Clone, Debug)]
pub enum ControlSignal {
    ConnectNewPeer(ConnectRequest),
    BroadcastMessage(Message),
}

#[derive(Clone, Debug)]
pub struct ConnectRequest {
    pub addr: std::net::SocketAddr,
    pub result_chan: crossbeam::channel::Sender<std::io::Result<BaseHandle>>,
}
