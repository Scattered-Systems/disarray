/*
   Appellation: behaviours <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{mainnet::*, testnet::*, utils::*};

pub(crate) mod mainnet;
pub(crate) mod testnet;

pub(crate) mod utils {
    use libp2p::kad::{AddProviderOk, KademliaEvent, PeerRecord, PutRecordOk, QueryResult, Record};
    use std::str::from_utf8;

    pub fn capture_kademlia_event(message: KademliaEvent) {
        if let KademliaEvent::OutboundQueryCompleted { result, .. } = message {
            match result {
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
            }
        }
    }
}
