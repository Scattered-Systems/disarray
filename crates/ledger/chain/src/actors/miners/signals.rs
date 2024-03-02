/*
   Appellation: signals <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::ContextUpdateSignal;
use crossbeam::channel::{unbounded, Receiver, Sender};

/// Bootstrap's a channel reciever and sender together under one structure
#[derive(Clone)]
pub struct SignalPack<T = ContextUpdateSignal> {
    pub receiver: Receiver<T>,
    pub sender: Sender<T>,
}

impl<T> SignalPack<T> {
    pub fn new(receiver: Receiver<T>, sender: Sender<T>) -> Self {
        Self { receiver, sender }
    }
    pub fn unbounded() -> Self {
        let (sender, recv) = unbounded();
        Self::new(recv, sender)
    }
}

impl<T> Default for SignalPack<T> {
    fn default() -> Self {
        Self::unbounded()
    }
}
