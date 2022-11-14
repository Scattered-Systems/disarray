/*
   Appellation: runtime <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::events::{Events, Event, Eventful};
use crate::procs::{SimpleProc, Processable};
use crate::{BoxResult, timestamp};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Runtime {
   pub event: Events,
   pub ledger: Vec<String>,
}

impl Runtime {
   pub fn new(event: Events, ledger: Vec<String>) -> Self {
      Self { event, ledger }
   }
   pub async fn spawn<T: Processable>(&mut self, proc: Arc<T>) -> BoxResult<&Self> {
      let mut i = 0;
      // Runtime Event Loop: Spawns typically unrelated processes on specified threads
      while i < 10 {
            let proc = Arc::clone(&proc);
            tokio::task::spawn(async move { 
               proc.spawn().await.expect("");
            });
            self.ledger
               .push(format!("(n.{:?}) Completed task at {:?}", i, timestamp()));
            i += 1;
      }
      Ok(self)
   }
}

impl std::convert::From<Events> for Runtime {
   fn from(data: Events) -> Self {
      Self::new(data, Default::default())
   }
}

impl Default for Runtime {
   fn default() -> Self {
      Self::new(Default::default(), Default::default())
   }
}

impl std::fmt::Display for Runtime {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::simple::SimpleProc;
   use std::sync::Arc;

   #[tokio::test]
   fn test_default_rt() {
      let a = Runtime::default();
      let b = SimpleProc::default();
      assert!(a.spawn(Arc::new(b)))
   }
}