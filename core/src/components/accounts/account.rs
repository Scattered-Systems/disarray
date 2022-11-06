/*
    Appellation: account <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Account {
    pub address: String,
}

impl Account {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}
