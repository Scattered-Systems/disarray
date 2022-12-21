/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{mainnet::*, testnet::*, utils::*};

pub(crate) mod mainnet;
pub(crate) mod testnet;

pub(crate) mod utils {
    use libp2p::kad::{AddProviderOk, KademliaEvent, PutRecordOk, QueryResult};
    use std::str::from_utf8;
    use tracing::info;

    pub fn capture_kademlia_event(message: KademliaEvent) {
        if let KademliaEvent::OutboundQueryProgressed { result, .. } = message {
            match result {
                QueryResult::GetProviders(Ok(v)) => {
                    info!("{:?}", v.clone());
                }
                QueryResult::GetProviders(Err(e)) => {
                    info!("{:?}", &e);
                }
                QueryResult::GetRecord(Ok(v)) => {
                    info!("{:?}", &v);
                }
                QueryResult::GetRecord(Err(e)) => {
                    info!("{:?}", &e);
                    eprintln!("Failed to get record: {:?}", e);
                }
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    info!("{:?}", &key);
                    println!(
                        "Successfully put record {:?}",
                        from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::PutRecord(Err(e)) => {
                    info!("{:?}", e);
                    eprintln!("Failed to put record: {:?}", e);
                }
                QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                    info!("{:?}", key);
                    println!(
                        "Successfully put provider record {:?}",
                        from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::StartProviding(Err(e)) => {
                    info!("{:?}", e);
                    eprintln!("Failed to put provider record: {:?}", e);
                }
                _ => {}
            }
        }
    }
}
