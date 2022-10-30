/*
   Appellation: utils <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::prelude::ring::{rand::SystemRandom, signature::Ed25519KeyPair};

pub fn random_keypair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref().into()).unwrap()
}
