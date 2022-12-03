/*
    Appellation: controller <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::ConnectRequest;
use scsys::prelude::Message;

pub enum ControlSignal {
    ConnectNewPeer(ConnectRequest),
    BroadcastMessage(Message),
    Idle,
}
