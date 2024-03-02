/*
    Appellation: provide <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::OneshotSender;

use libp2p::PeerId;
use std::collections::HashSet;

#[derive(Debug)]
pub struct GetProvider {
    pub fname: String,
    pub sender: OneshotSender<HashSet<PeerId>>,
}

impl GetProvider {
    pub fn new(fname: String, sender: OneshotSender<HashSet<PeerId>>) -> Self {
        Self { fname, sender }
    }
}
#[derive(Debug)]
pub struct StartProvider {
    pub fname: String,
    pub sender: OneshotSender<()>,
}

impl StartProvider {
    pub fn new(fname: String, sender: OneshotSender<()>) -> Self {
        Self { fname, sender }
    }
}
