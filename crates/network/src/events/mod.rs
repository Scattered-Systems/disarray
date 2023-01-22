/*
    Appellation: behaviours <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{eloop::*, event::*};

pub(crate) mod eloop;
pub(crate) mod event;

use crate::protocol::reqres::{MainnetRequest, MainnetResponse};

use libp2p::kad::KademliaEvent;
use libp2p::request_response;

#[derive(Debug)]
pub enum MainnetEvent {
    RequestResponse(request_response::RequestResponseEvent<MainnetRequest, MainnetResponse>),
    Kademlia(KademliaEvent),
}

impl From<request_response::RequestResponseEvent<MainnetRequest, MainnetResponse>>
    for MainnetEvent
{
    fn from(
        event: request_response::RequestResponseEvent<MainnetRequest, MainnetResponse>,
    ) -> Self {
        MainnetEvent::RequestResponse(event)
    }
}

impl From<KademliaEvent> for MainnetEvent {
    fn from(event: KademliaEvent) -> Self {
        MainnetEvent::Kademlia(event)
    }
}
