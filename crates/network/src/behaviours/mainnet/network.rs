/*
    Appellation: network <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::peers::Peer;
use crate::BoxedTransport;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NetworkType {
    #[default]
    Mainnet = 0,
    Testnet = 1,
}

pub struct Network {
    pub peer: Peer,
}

impl Network {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }
    pub fn transport(&self) -> BoxedTransport {
        crate::transports::tokio_transport(true, self.peer.keypair.clone())
    }
}
