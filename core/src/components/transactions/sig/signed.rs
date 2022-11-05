/*
   Appellation: signed <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use crate::transactions::{Sign, Transaction};
use scsys::crypto::hash::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SignedTransaction {
    pub sign: Sign,
    pub transaction: Transaction,
}

impl SignedTransaction {
    pub fn new(sign: Sign, transaction: Transaction) -> Self {
        Self { sign, transaction }
    }
}

impl std::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.sign, self.transaction)
    }
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
        hasher(self).as_slice().to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_transaction() {
        let signature = Sign::default();
        let transaction = Transaction::default();

        let a = SignedTransaction::new(signature, transaction);
        let b = SignedTransaction::default();
        assert_eq!(a, b)
    }
}