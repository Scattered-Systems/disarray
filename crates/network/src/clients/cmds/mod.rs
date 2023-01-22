/*
    Appellation: clients <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{dial::*, listen::*};

pub(crate) mod dial;
pub(crate) mod listen;

use crate::minis::reqres::MainnetResponse;

use clap::Subcommand;
use futures::channel::oneshot;
use libp2p::core::{Multiaddr, PeerId};
use libp2p::request_response::ResponseChannel;
use scsys::AsyncResult;
use std::collections::HashSet;

pub type SendError = Box<dyn std::error::Error + Send>;
pub type SendResult<T = ()> = Result<T, SendError>;
pub type OneshotSender<T = SendResult> = oneshot::Sender<T>;


#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    Dial(Dial),
    Listen(Listen)
}


#[derive(Debug)]
pub enum Command {
    StartListening {
        addr: Multiaddr,
        sender: OneshotSender,
    },
    Dial {
        peer_id: PeerId,
        peer_addr: Multiaddr,
        sender: OneshotSender,
    },
    StartProviding {
        file_name: String,
        sender: OneshotSender<()>,
    },
    GetProviders {
        file_name: String,
        sender: OneshotSender<HashSet<PeerId>>,
    },
    RequestFile {
        file_name: String,
        peer: PeerId,
        sender: OneshotSender<SendResult<Vec<u8>>>,
    },
    RespondFile {
        file: Vec<u8>,
        channel: ResponseChannel<MainnetResponse>,
    },
}