
use super::Transaction;

pub struct SignedTransaction {
    transaction: Transaction
}

impl SignedTransaction {
    pub fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }
}