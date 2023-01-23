/*
    Appellation: clients <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{dial::*, files::*, listen::*, provide::*};

pub(crate) mod dial;
pub(crate) mod files;
pub(crate) mod listen;
pub(crate) mod provide;

use crate::minis::reqres::MainnetResponse;
use futures::channel::oneshot;
use libp2p::request_response::ResponseChannel;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;

///
pub type SendError = Box<dyn std::error::Error + Send>;
///
pub type SendResult<T = ()> = Result<T, SendError>;
///
pub type OneshotSender<T = SendResult> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum Command {
    StartListening(Listen),
    Dial(Dial),
    StartProviding(StartProvider),
    GetProviders(GetProvider),
    RequestFile(FileRequest),
    RespondFile(FileResponse),
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: OneshotSender) -> Self {
        Self::Dial(Dial::new(addr, pid, sender))
    }
    pub fn listen(addr: Multiaddr, sender: OneshotSender) -> Self {
        Self::StartListening(Listen::new(addr, sender))
    }
    pub fn start_providing(fname: String, sender: OneshotSender<()>) -> Self {
        Self::StartProviding(StartProvider::new(fname, sender))
    }
    pub fn get_providers(fname: String, sender: OneshotSender<HashSet<PeerId>>) -> Self {
        Self::GetProviders(GetProvider::new(fname, sender))
    }
    pub fn request_file(
        addr: String,
        pid: PeerId,
        sender: OneshotSender<SendResult<Vec<u8>>>,
    ) -> Self {
        Self::RequestFile(FileRequest::new(addr, pid, sender))
    }
    pub fn respond_file(file: Vec<u8>, channel: ResponseChannel<MainnetResponse>) -> Self {
        Self::RespondFile(FileResponse::new(file, channel))
    }
}
