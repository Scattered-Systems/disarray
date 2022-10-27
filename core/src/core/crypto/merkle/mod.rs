/*
   Appellation: hash <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{layers::*, nodes::*, trees::*, utils::*};

pub(crate) mod layers;
pub(crate) mod nodes;
pub(crate) mod trees;

pub(crate) mod utils {
    use crate::crypto::hash::H256;
    use scsys::prelude::ring;
    use serde::Serialize;
    use sha2::{Digest, Sha256};

    pub fn shash<T: Serialize>(data: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&data).unwrap().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn merkle_hash<T: Serialize>(data: T) -> String {
        shash(shash(data))
    }

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
    }

    pub fn add_hash(a: &H256, b:&H256) -> H256 {
        let c = [a.as_ref(), b.as_ref()].concat();
        let combined = ring::digest::digest(&ring::digest::SHA256, &c);
        <H256>::from(combined)
    }
    
    /// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
    /// index of datum and `leaf_size`, the total number of leaves.
    pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
        let mut h: H256 = *datum;
        let proof_index = proof_path(index, leaf_size);
        for i in 0..proof.len() {
            if proof_index[i]%2==0 {
                h = add_hash(&proof[i], &h);
            } else {
                h = add_hash(&h, &proof[i]);
            }
        }
        *root == h
    }
    
    pub fn proof_path(index: usize, size: usize) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();
        let mut pos = index;
        let mut leaf_size = size;
        while leaf_size>1 {
            if leaf_size%2!=0 { leaf_size+=1; }
            if pos%2==0 {
                ans.push(pos+1);
            } else {
                ans.push(pos-1);
            }
            pos /= 2;
            leaf_size /= 2;
        }
        return ans;
    }
    
}
