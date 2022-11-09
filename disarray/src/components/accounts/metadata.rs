/*
    Appellation: metadata <account>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum AccountMetadata<T: Serialize = String> {
    Token {
        id: String,
        key: String,
        label: String,
        data: std::collections::HashMap<String, String>
    },
    #[strum(disabled)]
    Data(T),
    None
}

impl<T: Serialize> AccountMetadata<T> {
    
}

impl<T: Serialize> Default for AccountMetadata<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T: Serialize> std::fmt::Display for AccountMetadata<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_account_metadata() {
        let a = AccountMetadata::<String>::default();
        let b = AccountMetadata::None;
        assert_eq!(&a, &b)
    }
}
