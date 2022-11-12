/*
   Appellation: accounts <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{account::*, metadata::*, utils::*};

pub(crate) mod account;
pub(crate) mod metadata;

pub(crate) mod utils {
    use disarray::{compute_key_hash, file_to_vec};
    use scsys::prelude::{
        ring::signature::{Ed25519KeyPair, KeyPair},
        H160,
    };

    /// Creates a vector of accounts from the provided collection of keys
    pub fn create_ico_accounts(keys: Vec<Ed25519KeyPair>) -> Vec<H160> {
        keys.iter()
            .map(|i| compute_key_hash(i.public_key().as_ref().to_vec()).into())
            .collect::<Vec<H160>>()
    }
    /// Creates a vector of the given size composed of elligble keypairs
    pub fn create_ico_keys(n: usize) -> Vec<Ed25519KeyPair> {
        let lines: Vec<String> = file_to_vec("pubkeys.txt".to_string()).unwrap();

        let mut keys: Vec<Ed25519KeyPair> = Vec::new();
        for i in lines.iter().take(n) {
            let pkcs8_bytes = hex::decode(i.clone()).unwrap();
            let key = Ed25519KeyPair::from_pkcs8(&pkcs8_bytes[..]).unwrap();
            keys.push(key);
        }
        keys
    }
}
