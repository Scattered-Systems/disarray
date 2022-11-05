/*
   Appellation: sig <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{sign::*, signed::*, utils::*};

pub(crate) mod sign;
pub(crate) mod signed;

pub(crate) mod utils {
    use crate::transactions::{Sign, SignedTransaction, Transaction};
    use scsys::{
        crypto::hash::generate_random_hash,
        prelude::{
            rand::{self, Rng},
            ring::signature::{
                Ed25519KeyPair, EdDSAParameters, KeyPair, Signature, VerificationAlgorithm,
            },
        },
    };

    /// Create digital signature of a transaction
    pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
        let serialized: Vec<u8> = serde_json::to_vec(t).unwrap();
        key.sign(&serialized)
    }
}
