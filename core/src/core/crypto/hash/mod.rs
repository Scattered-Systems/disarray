/*
   Appellation: hash <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{hasher::*, hashes::*, utils::*};

pub(crate) mod hasher;
pub(crate) mod hashes;

pub trait Hashable<T: std::string::ToString = H256> {
    fn hash(&self) -> T;
}

pub(crate) mod utils {
    use super::H256;
    use scsys::prelude::rand::{self, Rng};
    use serde::Serialize;
    use sha2::{Digest, Sha256};
    use std::string::ToString;

    pub struct Hasher<T: Clone + Serialize, H: From<Vec<u8>> = H256> {
        pub data: T,
        pub hash: H
    }

    impl<T: Clone + Serialize, H: From<Vec<u8>>> Hasher<T, H> {
        pub fn new(data: T) -> Self {
            let hash: H = hasher(data.clone()).into();
            Self { data, hash }
        }
    }

    pub fn hasher<T: Serialize>(data: T) -> Vec<u8> {
        let mut hs = Sha256::new();
        hs.update(serde_json::to_string(&data).unwrap().as_bytes());
        hs.finalize().as_slice().to_owned().into()
    }

    pub fn block_hasher<T: Serialize>(data: T) -> H256 {
        let tmp = serde_json::to_string(&data).unwrap();
        let mut hasher = sha2::Sha256::new();
        hasher.update(tmp.as_bytes());
        hasher.finalize().as_slice().to_owned().into()
    }

    pub fn shash<T: ToString>(data: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn hash_to_degree_of<T: ToString>(data: T, degree: usize) -> String {
        let mut res = data.to_string();
        for _ in 0..degree {
            res = shash(res);
        }
        res
    }

    pub fn generate_random_hash() -> H256 {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }

    pub fn hash_divide_by(input: &H256, divide: f64) -> H256 {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
            //println!{"{}",value};
            let value = value as f64;
            let result = value / divide;
            let result = result as u32;
            let results: [u8; 4] = result.to_be_bytes();
            //println!{"{}",result};
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        (&result_bytes).into()
    }

    pub fn hash_multiply_by(input: &H256, multiply: f64) -> H256 {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
            //println!{"{}",value};
            let value = value as f64;
            let result = value * multiply;
            let result = result as u32;
            let results: [u8; 4] = result.to_be_bytes();
            //println!{"{}",result};
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        (&result_bytes).into()
    }
}
