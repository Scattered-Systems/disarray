/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;
use clap::Args;
use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Dial {
    #[arg(long, short)]
    pub addr: Multiaddr,
    #[arg(long, short)]
    pub peer_id: PeerId,
}

impl Dial {
    pub fn dial(&self, sender: OneshotSender) -> (Multiaddr, PeerId, OneshotSender) {
        (self.addr.clone(), self.peer_id.clone(), sender)
    }
}
