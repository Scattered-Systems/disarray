/*
    Appellation: machina <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Machine<T> {
    pub inputs: Vec<T>,
    pub outputs: Vec<String>,
}

impl<T> Machine<T> {
    pub fn new(inputs: Vec<T>) -> Self {
        Self {
            inputs,
            outputs: Default::default(),
        }
    }
}
