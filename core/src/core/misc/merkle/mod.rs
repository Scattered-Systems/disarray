/*
   Appellation: interface <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{interfaces::*, parts::*, proofs::*, shape::*, tree::*, utils::*};

pub(crate) mod interfaces;
pub(crate) mod parts;
pub(crate) mod proofs;
pub(crate) mod shape;
pub(crate) mod tree;

pub(crate) mod utils {
    use super::{proof_path, MerkleDimension, MerkleShape};
    use scsys::{
        crypto::hash::{Hashable, H256},
        prelude::ring,
    };

    pub fn add_hash(a: &H256, b: &H256) -> H256 {
        let c = [a.as_ref(), b.as_ref()].concat();
        let combined = ring::digest::digest(&ring::digest::SHA256, &c);
        <H256>::from(combined)
    }

    pub fn create_merkle_tree<T>(data: &[T]) -> (Box<dyn MerkleShape>, Vec<H256>)
    where
        T: Hashable,
    {
        // unimplemented!()
        let mut length = data.len();
        let mut nodes = Vec::new();
        let mut last_level = Vec::new();
        for i in data {
            let h: H256 = i.hash();
            last_level.push(h);
            nodes.push(h);
        }
        let mut depth = 1;
        while length > 1 {
            if length % 2 != 0 {
                last_level.push(data[length - 1].hash());
                nodes.push(data[length - 1].hash());
                length += 1;
            }
            let mut temp = Vec::new();
            for i in 0..length / 2 {
                let h: H256 = add_hash(&last_level[2 * i], &last_level[2 * i + 1]);
                temp.push(h);
                nodes.push(h);
            }
            last_level = temp.clone();
            length /= 2;
            depth += 1;
        }
        let dim = MerkleDimension::new(depth, data.len(), nodes.len());
        (Box::new(dim), nodes)
    }

    /// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
    /// index of datum and `leaf_size`, the total number of leaves.
    pub fn is_merkle_valid(
        root: &H256,
        datum: &H256,
        proof: &[H256],
        index: usize,
        leaf_size: usize,
    ) -> bool {
        let mut h: H256 = *datum;
        let proof_index = proof_path(index, leaf_size);
        for i in 0..proof.len() {
            if proof_index[i] % 2 == 0 {
                h = add_hash(&proof[i], &h);
            } else {
                h = add_hash(&h, &proof[i]);
            }
        }
        *root == h
    }
}

#[cfg(test)]
mod tests {
    /*
       Map(A -> a)
          def.
             This notation abbreviates a is the hash of A; more formally, (A) maps to the hash (a) by the hashing function H
    */
    use super::{is_merkle_valid, MerkleTree, MerkleTreeWrapper};
    use scsys::crypto::hash::{Hashable, H256};

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    macro_rules! gen_merkle_tree_data2 {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    /*
      A -> a: ("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d", "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0")
      B -> b: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
      C -> c: (concat(a, b), 6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920") where a ahead of b
    */
    #[test]
    fn test_merkle_root() {
        let data: Vec<H256> = gen_merkle_tree_data!();
        let expected =
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into();
        let a = MerkleTree::from(data);
        let root = a.root();
        assert_eq!(&root, &expected);
    }

    /*
      A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
    */
    #[test]
    fn test_merkle_proof() {
        let expected =
            vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()];

        let a = MerkleTree::from(gen_merkle_tree_data!());

        assert_eq!(a.proof(0), expected)
    }

    /*
       A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
    */
    #[test]
    fn show_proof() {
        let data: Vec<H256> = gen_merkle_tree_data2!();
        let merkle_tree = MerkleTree::from(data.clone());
        let index = 3;
        let proof = merkle_tree.proof(index);
        log::info!("{:?}", proof);
        assert!(is_merkle_valid(
            &merkle_tree.root(),
            &data[index].hash(),
            &proof,
            index,
            data.len()
        ));
        //  is the hash of
        //
    }

    #[test]
    fn test_vrf_tree() {
        let data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::from(data.clone());
        let proof = merkle_tree.proof(0);
        assert!(is_merkle_valid(
            &merkle_tree.root(),
            &data[0].hash(),
            &proof,
            0,
            data.len()
        ));
    }
}
