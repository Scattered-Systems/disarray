/*
    Appellation: responses <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::agents::Agency;
use scsys::prelude::{Message, Stateful, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Response<T: Display = Value> {
    pub agency: Agency,
    pub message: Message<T>,
    pub timestamp: i64,
}

impl<T: Display> Response<T> {
    pub fn new(agency: Agency, message: Message<T>) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            agency,
            message,
            timestamp,
        }
    }
}

impl<T: Clone + Default + Display + Serialize> Stateful for Response<T> {
    type Data = T;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl<T: Display + Serialize> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
