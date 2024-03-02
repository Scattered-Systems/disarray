/*
   Appellation: sign <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use decanter::prelude::{Hash, Hashable};
use serde::{Deserialize, Serialize};

pub trait SignatureWrapper {
    fn public_key(&self) -> Vec<u8>;
    fn signature(&self) -> Vec<u8>;
}

/// This structure models the expected signature
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sign {
    pub pubk: Vec<u8>,
    pub sig: Vec<u8>,
}

impl Sign {
    pub fn new(pubk: Vec<u8>, sig: Vec<u8>) -> Self {
        Self { pubk, sig }
    }
}

impl SignatureWrapper for Sign {
    fn public_key(&self) -> Vec<u8> {
        self.pubk.clone()
    }
    fn signature(&self) -> Vec<u8> {
        self.sig.clone()
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.pubk, self.sig)
    }
}
