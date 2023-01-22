/*
    Appellation: listen <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;
use clap::Args;
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Listen {
    #[arg(long, short)]
    pub addr: Multiaddr,
}

impl Listen {
    pub fn start_listening(&self, sender: OneshotSender) -> (Multiaddr, OneshotSender) {
        (self.addr.clone(), sender)
    }
}
