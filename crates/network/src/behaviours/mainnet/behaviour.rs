/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::MainnetEvent;
use crate::protocol::codec::MainnetCodec;
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;


#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MainnetEvent")]
pub struct MainnetBehaviour {
    pub reqres: request_response::RequestResponse<MainnetCodec>,
    pub kademlia: Kademlia<MemoryStore>,
}

impl MainnetBehaviour {
    // Get an owned instance of the Kademlia agents
    pub fn kademlia(&self) -> &Kademlia<MemoryStore> {
        &self.kademlia
    }
    // Get an owned instance of the Resquest / Response agents
    pub fn request_response(&self) -> &request_response::RequestResponse<MainnetCodec> {
        &self.reqres
    }
}


