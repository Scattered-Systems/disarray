/*
   Appellation: recorder <spam>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use std::collections::HashSet;

use crate::transactions::{SpamId, SignedTransaction};

#[derive(Debug)]
pub struct SpamRecorder {
    set: HashSet<SpamId>
}

impl SpamRecorder {
    pub fn new() -> Self {
        Self {
            set: HashSet::new()
        }
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

