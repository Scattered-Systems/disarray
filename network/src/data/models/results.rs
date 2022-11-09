/*
    Appellation: results <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub enum ReadResult {
    Continue,
    Message(Vec<u8>),
    EOF,
}

pub enum WriteResult {
    Complete,
    EOF,
    ChanClosed,
}