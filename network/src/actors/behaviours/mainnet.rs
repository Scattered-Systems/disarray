/*
    Appellation: mainnet <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::KademliaMS;

use libp2p::{floodsub::{Floodsub, FloodsubEvent}, kad::KademliaEvent, swarm::NetworkBehaviour};
use libp2p::mdns::Event as MdnsEvent;
use libp2p::mdns::tokio::Behaviour as TokioMdns;


/// Create the standard behaviour for blockchain networks, building on top of Kademlia and MDNS
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MainnetBehaviourEvent")]
pub struct MainnetBehaviour {
    pub floodsub: Floodsub,
    pub kademlia: KademliaMS,
    pub mdns: TokioMdns,
}

pub enum MainnetBehaviourEvent {
    Fsub(FloodsubEvent),
    Kad(KademliaEvent),
    Mdns(MdnsEvent),
}

impl From<FloodsubEvent> for MainnetBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self {
        Self::Fsub(event)
    }
}

impl From<KademliaEvent> for MainnetBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        Self::Kad(event)
    }
}

impl From<MdnsEvent> for MainnetBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[tokio::test]
    async fn test_mainnet() {
        assert!(true)
    }
}