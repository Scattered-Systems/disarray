/*
   Appellation: recorder <spam>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::transactions::{SignedTransaction, SpamId};
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
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

