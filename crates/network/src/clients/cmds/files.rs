/*
    Appellation: files <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{OneshotSender, SendResult};
use crate::minis::reqres::MainnetResponse;
use clap::{Args, ValueEnum};
use libp2p::request_response::ResponseChannel;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ValueEnum,
)]
pub enum ReqRes {
    #[default]
    Request = 0,
    Response = 1,
}

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FileRequest {
    #[arg(long, short)]
    pub fname: String,
    #[arg(long, short)]
    pub pid: PeerId,
}

impl FileRequest {
    pub fn request(
        &self,
        sender: OneshotSender<SendResult<Vec<u8>>>,
    ) -> (String, PeerId, OneshotSender<SendResult<Vec<u8>>>) {
        (self.fname.clone(), self.pid.clone(), sender)
    }
}

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FileResponse {
    #[arg(long, short)]
    pub file: Vec<u8>,
}

impl FileResponse {
    pub fn response(
        &self,
        channel: ResponseChannel<MainnetResponse>,
    ) -> (Vec<u8>, ResponseChannel<MainnetResponse>) {
        (self.file.clone(), channel)
    }
}
