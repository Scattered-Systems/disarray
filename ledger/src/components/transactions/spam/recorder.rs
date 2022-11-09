/*
   Appellation: recorder <spam>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::{SignedTransaction, SpamId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SpamRecorder {
    set: HashSet<SpamId>,
}

impl SpamRecorder {
    pub fn new(set: HashSet<SpamId>) -> Self {
        Self { set }
    }
    /// return false if the element is already in
    pub fn test(&self, t: &SignedTransaction) -> bool {
        !self.set.contains(&(t.into()))
    }
    /// return false if the element is already in
    pub fn test_and_set(&mut self, t: &SignedTransaction) -> bool {
        self.set.insert(t.into())
    }
}

impl std::fmt::Display for SpamRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
