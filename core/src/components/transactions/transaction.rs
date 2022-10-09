/*
   Appellation: transaction <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use scsys::prelude::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub enum TransactionState {
    Signed,
    Unsigned,

}
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Transaction {
    pub id: ObjectId,
    pub label: Option<String>,
    pub data: Vec<String>,
}

impl Transaction {
    pub fn new(label: Option<String>, data: Vec<String>) -> Self {
        let id = ObjectId::new();
        Self { id, label, data, }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new(None, Vec::new())
    }
}
