/*
    Appellation: server <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::peers::Handle;
use crossbeam::channel as cbchannel;
use mio_extras::channel;
use scsys::prelude::Message;

pub enum ControlSignal {
    ConnectNewPeer(ConnectRequest),
    BroadcastMessage(Message),
    Idle,
}

pub struct ConnectRequest {
    addr: std::net::SocketAddr,
    result_chan: cbchannel::Sender<std::io::Result<Handle>>,
}

#[derive(Clone)]
pub struct ServerHandle {
    control_chan: channel::Sender<ControlSignal>,
}

impl ServerHandle {
    pub fn new(control_chan: channel::Sender<ControlSignal>) -> Self {
        Self { control_chan }
    }
    pub fn connect(&self, addr: std::net::SocketAddr) -> std::io::Result<Handle> {
        let (sender, receiver) = cbchannel::unbounded();
        let request = ConnectRequest {
            addr,
            result_chan: sender,
        };
        self.control_chan
            .send(ControlSignal::ConnectNewPeer(request))
            .unwrap();
        receiver.recv().unwrap()
    }

    pub fn broadcast(&self, msg: Message) {
        self.control_chan
            .send(ControlSignal::BroadcastMessage(msg))
            .unwrap();
    }
}

impl Default for ServerHandle {
    fn default() -> Self {
        let (s, _) = mio_extras::channel::channel();
        Self::new(s)
    }
}
