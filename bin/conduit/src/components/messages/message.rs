/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Message<T: fmt::Display = Value> {
    pub data: Vec<T>,
    pub timestamp: i64,
}

impl<T: fmt::Display> Message<T> {
    pub fn new(data: Vec<T>) -> Self {
        let timestamp = Timestamp::default().into();
        Self { data, timestamp }
    }
    pub fn content(&self) -> &Vec<T> {
        &self.data
    }
    pub fn push(&mut self, data: T) -> &Self {
        self.data.push(data);
        self
    }
}

impl<T: fmt::Display> std::convert::From<Vec<T>> for Message<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

impl<T: fmt::Display> std::convert::From<T> for Message<T> {
    fn from(data: T) -> Self {
        Self::new(vec![data])
    }
}

impl<T: Serialize + fmt::Display> std::fmt::Display for Message<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap().to_lowercase()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message() {
        let a = Message::<String>::default();
        assert_eq!(&a.data, &a.data)
    }
}
