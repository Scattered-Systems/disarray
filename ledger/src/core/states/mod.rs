/*
    Appellation: states <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{specs::*, state::*, utils::*};

pub(crate) mod state;

pub(crate) mod specs {
    use crate::BlockState;

    pub trait BlockchainStateWrapper {
        fn blockstate(&self) -> BlockState;
    }
}

pub(crate) mod utils {
    use crate::{
        transactions::{verify_signedtxn, SignedTransaction},
        StateMap,
    };
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use scsys::prelude::{H160, H256};
    use std::io::{BufRead, BufReader};

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
    /// A function wrapper converting the given vector of elements type u8
    pub fn compute_key_hash(key: Vec<u8>) -> H256 {
        key.into()
    }
    /// From the given path, open the file and gathers its contents into a vector
    pub fn file_to_vec(filename: String) -> std::io::Result<Vec<String>> {
        let file_in = std::fs::File::open(filename)?;
        let file_reader = BufReader::new(file_in);
        Ok(file_reader
            .lines()
            .filter_map(std::io::Result::ok)
            .collect())
    }
    /// Check the given transaction against the provided state-map
    pub fn transaction_check(current_state: &mut StateMap, tx: &SignedTransaction) -> bool {
        if verify_signedtxn(tx) {
            let copy = tx.clone();
            let pubk = copy.sign.pubk.clone();
            let nonce = copy.transaction.nonce;
            let value = copy.transaction.value;
            let recv = copy.transaction.recv;

            let sender: H160 = compute_key_hash(pubk).into();
            let (s_nonce, s_amount) = *current_state.get(&sender).unwrap();
            let (r_nonce, r_amount) = *current_state.get(&recv).unwrap();

            if nonce == s_nonce + 1 && s_amount >= value {
                current_state.insert(sender, (s_nonce + 1, s_amount - value));
                current_state.insert(recv, (r_nonce, r_amount + value));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
