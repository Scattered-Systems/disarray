/*
    Appellation: provide <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;
use clap::Args;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Provide {
    #[arg(long, short)]
    pub fname: String,
}

impl Provide {
    pub fn file_name(&self) -> String {
        self.fname.clone()
    }
    pub fn get(
        &self,
        sender: OneshotSender<HashSet<PeerId>>,
    ) -> (String, OneshotSender<HashSet<PeerId>>) {
        (self.file_name(), sender)
    }
    pub fn start(&self, sender: OneshotSender<()>) -> (String, OneshotSender<()>) {
        (self.file_name(), sender)
    }
}
