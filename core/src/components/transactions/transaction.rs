/*
   Appellation: transaction <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use scsys::BsonOid;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub id: BsonOid,
    pub label: Option<String>,
    pub data: Vec<String>
}


impl Transaction {
    pub fn new(label: Option<String>, data: Vec<String>) -> Self {
        Self { id: BsonOid::new(), label, data }
    }
    pub fn sign_transaction(&self, sig: String) {
        todo!()
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new(None, Vec::new())
    }
}


#[cfg(test)]
mod tests {
    use super::Transaction;

    #[test]
    fn test_transaction() {
        let a = Transaction::default();
        let b = Transaction::new(None, Vec::new());
        assert_ne!(a, b)
    }
}