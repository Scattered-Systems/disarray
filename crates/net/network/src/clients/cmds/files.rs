/*
    Appellation: files <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{OneshotSender, SendResult};
use crate::minis::reqres::MainnetResponse;
use clap::ValueEnum;
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

#[derive(Debug)]
pub struct FileRequest {
    pub fname: String,
    pub pid: PeerId,
    pub sender: OneshotSender<SendResult<Vec<u8>>>,
}

impl FileRequest {
    pub fn new(fname: String, pid: PeerId, sender: OneshotSender<SendResult<Vec<u8>>>) -> Self {
        Self { fname, pid, sender }
    }
}

#[derive(Debug)]
pub struct FileResponse {
    pub file: Vec<u8>,
    pub channel: ResponseChannel<MainnetResponse>,
}

impl FileResponse {
    pub fn new(file: Vec<u8>, channel: ResponseChannel<MainnetResponse>) -> Self {
        Self { file, channel }
    }
}
