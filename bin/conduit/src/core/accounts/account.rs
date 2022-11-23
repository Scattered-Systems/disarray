/*
    Appellation: account <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::AccountMetadata;
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Account {
    pub address: String,
    pub metadata: Vec<AccountMetadata>,
    pub modified: i64, // current time
    pub timestamp: i64,
}

impl Account {
    pub fn new(address: String, metadata: Vec<AccountMetadata>, modified: i64) -> Self {
        let timestamp = Timestamp::ts();
        Self {
            address,
            metadata,
            modified,
            timestamp,
        }
    }
}

impl std::convert::From<String> for Account {
    fn from(data: String) -> Self {
        Self::new(data, Default::default(), Timestamp::ts())
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_account() {
        let a = Account::default();
        let b = Account::from(String::new());
        assert_eq!(&a.address, &b.address)
    }
}
