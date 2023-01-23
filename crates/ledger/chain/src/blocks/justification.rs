/*
   Appellation: justification <header>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::blocks::Verifiable;
use decanter::prelude::{Hash, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockJustification {
    pub hash: Vec<u8>,
    pub proof: Vec<u8>,
    pub pub_key: Vec<u8>,
}

impl BlockJustification {
    pub fn new(hash: Vec<u8>, proof: Vec<u8>, pub_key: Vec<u8>) -> Self {
        Self {
            hash,
            proof,
            pub_key,
        }
    }
}

impl Verifiable for BlockJustification {
    fn vrf_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
    fn vrf_proof(&self) -> Vec<u8> {
        self.proof.clone()
    }
    fn vrf_pub_key(&self) -> Vec<u8> {
        self.pub_key.clone()
    }
}

impl std::convert::From<(Vec<u8>, Vec<u8>, Vec<u8>)> for BlockJustification {
    fn from(data: (Vec<u8>, Vec<u8>, Vec<u8>)) -> Self {
        Self::new(data.0, data.1, data.2)
    }
}

impl std::convert::From<(&[u8], &[u8], &[u8])> for BlockJustification {
    fn from(data: (&[u8], &[u8], &[u8])) -> Self {
        Self::new(data.0.to_owned(), data.1.to_owned(), data.2.to_owned())
    }
}

impl std::fmt::Display for BlockJustification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_justification() {
        let a = BlockJustification::default();
        let b = BlockJustification::new(Default::default(), Default::default(), Default::default());
        assert_eq!(&a, &b)
    }
}
