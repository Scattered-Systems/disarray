/*
   Appellation: keys <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use scsys::prelude::ring::{rand::SystemRandom, signature::Ed25519KeyPair};

/// Generate a random key pair.
pub fn random() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref().into()).unwrap()
}