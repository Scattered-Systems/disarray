/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::events::MainnetEvent;
use crate::minis::codec::MainnetCodec;
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response::RequestResponse;
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MainnetEvent")]
pub struct MainnetBehaviour {
    pub reqres: RequestResponse<MainnetCodec>,
    pub kademlia: Kademlia<MemoryStore>,
}

impl MainnetBehaviour {
    pub fn new(reqres: RequestResponse<MainnetCodec>, kademlia: Kademlia<MemoryStore>) -> Self {
        Self { reqres, kademlia }
    }
    // Get an owned instance of the Kademlia agents
    pub fn kademlia(&self) -> &Kademlia<MemoryStore> {
        &self.kademlia
    }
    // Get an owned instance of the Resquest / Response agents
    pub fn request_response(&self) -> &RequestResponse<MainnetCodec> {
        &self.reqres
    }
}
