/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::protocol::reqres::MainnetResponse;

use libp2p::request_response::ResponseChannel;

#[derive(Debug)]
pub enum Event {
    InboundRequest {
        request: String,
        channel: ResponseChannel<MainnetResponse>,
    },
}
