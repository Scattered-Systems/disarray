/*
   Appellation: utils <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::prelude::ring::{rand::SystemRandom, signature::Ed25519KeyPair};

pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res.into_bytes()
}

pub fn random_keypair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap()
}
