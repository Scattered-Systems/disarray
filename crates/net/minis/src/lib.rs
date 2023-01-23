/*
    Appellation: disarray-minis <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub mod block_fetch;
pub mod chain_sync;
pub mod codec;
pub mod handshake;
pub mod n2n;
pub mod ping_pong;
pub mod reqres;
pub mod submit_local_transactions;

use libp2p::core::upgrade::ProtocolName;

#[derive(Debug, Clone)]
pub struct MainnetProtocol();

impl ProtocolName for MainnetProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/disarray/9991".as_bytes()
    }
}
