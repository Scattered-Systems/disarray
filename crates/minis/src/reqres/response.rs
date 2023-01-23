/*
    Appellation: response <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MainnetResponse(pub Vec<u8>);

impl MainnetResponse {
    pub fn response(&self) -> Vec<u8> {
        self.0.clone()
    }
}
