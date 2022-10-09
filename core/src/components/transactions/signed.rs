/*
   Appellation: signed <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SignedTransaction {
    transaction: Transaction
}

impl SignedTransaction {
    pub fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }
}