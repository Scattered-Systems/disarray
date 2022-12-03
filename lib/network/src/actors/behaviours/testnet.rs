/*
    Appellation: testnet <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{self, tokio::Behaviour as TokioMdns},
    swarm::NetworkBehaviour,
};

/// Create the standard behaviour for blockchain networks, building on top of Kademlia and MDNS
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "TestnetBehaviourEvent")]
pub struct TestnetBehaviour {
    pub floodsub: Floodsub,
    pub mdns: TokioMdns,
}

pub enum TestnetBehaviourEvent {
    Fsub(FloodsubEvent),
    Mdns(mdns::Event),
}

impl From<FloodsubEvent> for TestnetBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self {
        Self::Fsub(event)
    }
}

impl From<mdns::Event> for TestnetBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}
