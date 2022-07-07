/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use block::*;

mod block;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transaction<Id = u8, Key = String, Ts = i64, Data = String> {
    id: Id,
    key: Key,
    timestamp: Ts,
    data: Vec<Data>,
}

impl<Id, Key, Ts, Data> Transaction<Id, Key, Ts, Data> {
    pub fn constructor(id: Id, key: Key, timestamp: Ts, data: Vec<Data>) -> Self {
        Self { id, key, timestamp, data }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction(\nid={:#?},\nkey={:#?},\ntimestamp={:#?},\ndata={:#?}\n)",
            self.id, self.key, self.timestamp, self.data
        )
    }
}