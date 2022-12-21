/*
    Appellation: server <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::handles::{peers::Handle, servers::*};
use mio_extras::channel as mc;
use scsys::prelude::Message;

#[derive(Clone)]
pub struct ServerHandle {
    channel: mc::Sender<ControlSignal>,
}

impl ServerHandle {
    pub fn new(channel: mc::Sender<ControlSignal>) -> Self {
        Self { channel }
    }
    pub fn connect(&self, addr: std::net::SocketAddr) -> std::io::Result<Handle> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        let request = ConnectRequest::new(addr, sender);
        self.channel
            .send(ControlSignal::ConnectNewPeer(request))
            .unwrap();
        receiver.recv().unwrap()
    }

    pub fn broadcast(&self, msg: Message) {
        self.channel
            .send(ControlSignal::BroadcastMessage(msg))
            .unwrap();
    }
}

impl Default for ServerHandle {
    fn default() -> Self {
        let (s, _) = mc::channel();
        Self::new(s)
    }
}

impl std::convert::From<&ServerHandle> for ServerHandle {
    fn from(data: &ServerHandle) -> Self {
        data.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_server_handle_default() {
        let a = ServerHandle::default();
        let _b = ServerHandle::from(&a);

        assert!(true)
    }
}
