/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub mod codec;
pub mod reqres;

use libp2p::core::upgrade::ProtocolName;

#[derive(Debug, Clone)]
pub struct MainnetProtocol();

impl ProtocolName for MainnetProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/disarray/9991".as_bytes()
    }
}


