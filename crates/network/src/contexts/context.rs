/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::handlers::BaseHandle;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Direction {
    #[default]
    Incoming = 0,
    Outgoing = 1,
}

pub struct Context {
    pub addr: std::net::SocketAddr,
    pub direction: Direction,
    pub handle: BaseHandle,
    pub stream: mio::net::TcpStream,
}

impl Context {
    pub fn setup(direction: Direction, stream: std::net::TcpStream) -> Self {
        Self::from((direction, stream))
    }
}

impl std::convert::From<(Direction, std::net::TcpStream)> for Context {
    fn from(data: (Direction, std::net::TcpStream)) -> Self {
        let (direction, stream) = data;
        let addr = stream
            .peer_addr()
            .expect("Failed to fetch the peers address");

        let handle = BaseHandle {};
        Self {
            addr,
            stream: mio::net::TcpStream::from_std(stream),
            handle,
            direction,
        }
    }
}
