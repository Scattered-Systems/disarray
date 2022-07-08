/*
    Appellation: mainnet
    Context: behaviours::ipfs
    Description:
        Implement the network behaviours that enables apps to connect to the ipfs mainnet

*/
use crate::KademliaMS;
use libp2p::{
    kad::{AddProviderOk, KademliaEvent, PeerRecord, PutRecordOk, QueryResult, Record},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use std::str::from_utf8;

pub fn capture_kademlia_event(message: KademliaEvent) {
    match message {
        KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
            QueryResult::GetProviders(Ok(ok)) => {
                for peer in ok.providers {
                    println!(
                        "Peer {:?} provides key {:?}",
                        peer,
                        from_utf8(ok.key.as_ref()).unwrap()
                    );
                }
            }
            QueryResult::GetProviders(Err(err)) => {
                eprintln!("Failed to get providers: {:?}", err);
            }
            QueryResult::GetRecord(Ok(ok)) => {
                for PeerRecord {
                    record: Record { key, value, .. },
                    ..
                } in ok.records
                {
                    println!(
                        "Got record {:?} {:?}",
                        from_utf8(key.as_ref()).unwrap(),
                        from_utf8(&value).unwrap()
                    );
                }
            }
            QueryResult::GetRecord(Err(err)) => {
                eprintln!("Failed to get record: {:?}", err);
            }
            QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                println!(
                    "Successfully put record {:?}",
                    from_utf8(key.as_ref()).unwrap()
                );
            }
            QueryResult::PutRecord(Err(err)) => {
                eprintln!("Failed to put record: {:?}", err);
            }
            QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                println!(
                    "Successfully put provider record {:?}",
                    from_utf8(key.as_ref()).unwrap()
                );
            }
            QueryResult::StartProviding(Err(err)) => {
                eprintln!("Failed to put provider record: {:?}", err);
            }
            _ => {}
        },
        _ => {}
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct Mainnet {
    pub kademlia: KademliaMS,
}

impl NetworkBehaviourEventProcess<KademliaEvent> for Mainnet {
    fn inject_event(&mut self, message: KademliaEvent) {
        capture_kademlia_event(message)
    }
}
