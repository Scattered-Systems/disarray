/*
   Appellation: h160 <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
use super::h256::H256;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct H160(pub [u8; 20]); // big endian u256

impl std::convert::From<&[u8; 20]> for H160 {
    fn from(input: &[u8; 20]) -> H160 {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(input);
        H160(buffer)
    }
}

impl std::convert::From<[u8; 20]> for H160 {
    fn from(input: [u8; 20]) -> H160 {
        H160(input)
    }
}

impl std::convert::From<&H256> for H160 {
    fn from(input: &H256) -> H160 {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl std::convert::From<H256> for H160 {
    fn from(input: H256) -> H160 {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl std::fmt::Debug for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[18], &self.0[19]
        )
    }
}