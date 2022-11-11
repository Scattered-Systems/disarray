/*
    Appellation: requests <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{agents::Agency, messages::Message, states::Stateful};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Request<T: Display = Value> {
    pub agency: Agency,
    pub message: Message<T>,
    pub timestamp: i64,
}

impl<T: Display> Request<T> {
    pub fn new(agency: Agency, message: Message<T>) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            agency,
            message,
            timestamp,
        }
    }
}

impl<T: Clone + Default + Display + Serialize> Stateful for Request<T> {
    type Data = T;

    fn agency(&self) -> String {
        self.agency.to_string()
    }
    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl<T: Display + Serialize> Display for Request<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
