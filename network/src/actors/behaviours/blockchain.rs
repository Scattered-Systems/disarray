/*
    Appellation: blockchain <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
};

use crate::KademliaMS;

/// Create the standard behaviour for blockchain networks, building on top of Kademlia and MDNS
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MainnetEvent")]
pub struct BlockchainMainnet {
    pub mdns: Mdns,
}

pub enum MainnetEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for MainnetEvent {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}
