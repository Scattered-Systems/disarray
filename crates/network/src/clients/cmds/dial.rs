/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;
use libp2p::{Multiaddr, PeerId};

#[derive(Debug)]
pub struct Dial {
    pub addr: Multiaddr,
    pub peer_id: PeerId,
    pub sender: OneshotSender,
}

impl Dial {
    pub fn new(addr: Multiaddr, peer_id: PeerId, sender: OneshotSender) -> Self {
        Self {
            addr,
            peer_id,
            sender,
        }
    }
}
