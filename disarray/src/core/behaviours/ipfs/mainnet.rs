/*
   Appellation: mainnet <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::KademliaMS;
use libp2p::{
    kad::KademliaEvent,
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use std::str::from_utf8;


#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct IPFSMainnet {
    pub kademlia: KademliaMS,
}

impl NetworkBehaviourEventProcess<KademliaEvent> for IPFSMainnet {
    fn inject_event(&mut self, message: KademliaEvent) {
        crate::capture_kademlia_event(message)
    }
}
