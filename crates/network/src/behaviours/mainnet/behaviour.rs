/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::MainnetEvent;
use async_trait::async_trait;
use futures::prelude::*;
use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName};
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;
use tokio::io;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MainnetEvent")]
pub struct MainnetBehaviour {
    pub reqres: request_response::RequestResponse<MainnetCodec>,
    pub kademlia: Kademlia<MemoryStore>,
}

impl MainnetBehaviour {
    // Get an owned instance of the Kademlia agents
    pub fn kademlia(&self) -> &Kademlia<MemoryStore> {
        &self.kademlia
    }
    // Get an owned instance of the Resquest / Response agents
    pub fn request_response(&self) -> &request_response::RequestResponse<MainnetCodec> {
        &self.reqres
    }
}

// Simple file exchange protocol

#[derive(Debug, Clone)]
pub struct MainnetProtocol();
#[derive(Clone)]
pub struct MainnetCodec();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainnetRequest(pub(crate) String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainnetResponse(pub(crate) Vec<u8>);

impl ProtocolName for MainnetProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/disarray/9991".as_bytes()
    }
}

#[async_trait]
impl request_response::RequestResponseCodec for MainnetCodec {
    type Protocol = MainnetProtocol;
    type Request = MainnetRequest;
    type Response = MainnetResponse;

    async fn read_request<T>(
        &mut self,
        _: &MainnetProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 1_000_000).await?;

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(MainnetRequest(String::from_utf8(vec).unwrap()))
    }

    async fn read_response<T>(
        &mut self,
        _: &MainnetProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 500_000_000).await?; // update transfer maximum

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(MainnetResponse(vec))
    }

    async fn write_request<T>(
        &mut self,
        _: &MainnetProtocol,
        io: &mut T,
        MainnetRequest(data): MainnetRequest,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, data).await?;
        io.close().await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &MainnetProtocol,
        io: &mut T,
        MainnetResponse(data): MainnetResponse,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, data).await?;
        io.close().await?;

        Ok(())
    }
}
