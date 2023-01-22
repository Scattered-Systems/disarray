/*
    Appellation: codec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::protocol::{
    reqres::{MainnetRequest, MainnetResponse},
    MainnetProtocol,
};
use async_trait::async_trait;
use futures::prelude::*;
use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed};
use libp2p::request_response;
use tokio::io;

#[derive(Clone)]
pub struct MainnetCodec();

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
