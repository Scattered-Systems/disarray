/*
    Appellation: listen <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;
use libp2p::Multiaddr;

#[derive(Debug)]
pub struct Listen {
    pub addr: Multiaddr,
    pub sender: OneshotSender,
}

impl Listen {
    pub fn new(addr: Multiaddr, sender: OneshotSender) -> Self {
        Self { addr, sender }
    }
}
