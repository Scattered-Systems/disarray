/*
    Appellation: mainnet
    Context: behaviours::ipfs
    Description:
        Implement the network behaviours that enables apps to connect to the ipfs mainnet

*/

use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    identify::{Identify, IdentifyEvent},
    ping,
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct Subnet {
    pub gossipsub: Gossipsub,
    pub identify: Identify,
    pub ping: ping::Behaviour,
}

// Define the network's identification behaviour
impl NetworkBehaviourEventProcess<IdentifyEvent> for Subnet {
    // Called when `identify` produces an event.
    fn inject_event(&mut self, event: IdentifyEvent) {
        println!("identify: {:?}", event);
    }
}

// Implement the network's Gossipsub Behaviour
impl NetworkBehaviourEventProcess<GossipsubEvent> for Subnet {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => println!(
                "Got message: {} with id: {} from peer: {:#?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            ),
            _ => {}
        }
    }
}

// Define the network's ping behaviour
impl NetworkBehaviourEventProcess<ping::Event> for Subnet {
    // Called when `ping` produces an event.
    fn inject_event(&mut self, event: ping::Event) {
        match event {
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Ping { rtt }),
            } => {
                println!(
                    "ping: rtt to {} is {} ms",
                    peer.to_base58(),
                    rtt.as_millis()
                );
            }
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Pong),
            } => {
                println!("ping: pong from {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Timeout),
            } => {
                println!("ping: timeout to {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Unsupported),
            } => {
                println!("ping: {} does not support ping protocol", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Other { error }),
            } => {
                println!("ping: ping::Failure with {}: {}", peer.to_base58(), error);
            }
        }
    }
}
