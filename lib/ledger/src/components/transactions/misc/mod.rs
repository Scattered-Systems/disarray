/*
   Appellation: spam <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{recorder::*, sign::*, spam_id::*, specs::*, utils::*};

pub(crate) mod recorder;
pub(crate) mod sign;
pub(crate) mod spam_id;

pub(crate) mod specs {
    use crate::{BlockNc, BlockTs};
    use scsys::prelude::H160;
    use std::string::ToString;

    pub trait Transactable<T: ToString> {
        fn message(&self) -> &T;
        fn nonce(&self) -> BlockNc;
        fn recv(&self) -> H160;
        fn timestamp(&self) -> BlockTs;
        fn value(&self) -> usize;
    }

    pub trait TransactionWrapper<T: ToString>: Transactable<T> {}

    pub trait TransactionWrapperExt<T: ToString>: TransactionWrapper<T> {}
}

pub(crate) mod utils {
    use ring::{
        error::KeyRejected,
        signature::{self, Ed25519KeyPair},
    };

    pub fn keypair_from_pkcs8(pkcs8: &[u8]) -> Result<Ed25519KeyPair, KeyRejected> {
        Ed25519KeyPair::from_pkcs8(pkcs8)
    }

    pub fn sign<T>(kp: &Ed25519KeyPair, msg: &T) -> signature::Signature
    where
        T: ToString,
    {
        kp.sign(msg.to_string().as_bytes())
    }
}
