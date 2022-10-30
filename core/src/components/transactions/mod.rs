/*
   Appellation: transactions <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{misc::*, signed::*, transaction::*, utils::*};

pub(crate) mod misc;
pub(crate) mod signed;
pub(crate) mod transaction;

pub type Transactions = Vec<Transaction>;

pub(crate) mod utils {
    use super::{Sign, SignedTransaction, Transaction};
    use scsys::{
        crypto::hash::generate_random_hash,
        prelude::{
            rand::{self, Rng},
            ring::signature::{
                Ed25519KeyPair, EdDSAParameters, KeyPair, Signature, VerificationAlgorithm,
            },
        },
    };

    /// Create digital signature of a transaction
    pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
        let serialized: Vec<u8> = serde_json::to_vec(t).unwrap();
        key.sign(&serialized)
    }

    /// Verify digital signature of a transaction, using public key instead of secret key
    pub fn verify(
        t: &Transaction,
        public_key: &<Ed25519KeyPair as KeyPair>::PublicKey,
        signature: &Signature,
    ) -> bool {
        let serialized: Vec<u8> = serde_json::to_vec(t).unwrap();
        let bytes: &[u8] = &serialized;
        match VerificationAlgorithm::verify(
            &EdDSAParameters,
            public_key.as_ref().into(),
            bytes.into(),
            signature.as_ref().into(),
        ) {
            Ok(_) => true,
            Err(_e) => false,
        }
    }

    pub fn verify_signedtxn(t: &SignedTransaction) -> bool {
        let transaction = t.transaction.clone();
        let pubk = t.sign.pubk.clone();
        let sig = t.sign.sig.clone();
        let serialized: Vec<u8> = serde_json::to_vec(&transaction).unwrap();
        let bytes: &[u8] = &serialized;
        match VerificationAlgorithm::verify(
            &EdDSAParameters,
            pubk[..].into(),
            bytes.into(),
            sig[..].into(),
        ) {
            Ok(_) => true,
            Err(_e) => false,
        }
    }

    pub fn generate_random_transaction() -> Transaction {
        let mut rng = rand::thread_rng();
        Transaction::new(rng.gen(), generate_random_hash().into(), rng.gen())
    }

    pub fn generate_random_signed_transaction() -> SignedTransaction {
        let transaction = generate_random_transaction();
        let pubk = crate::random_keypair();
        let sig = sign(&transaction, &pubk);
        let sign = Sign {
            pubk: pubk.public_key().as_ref().to_vec(),
            sig: sig.as_ref().to_vec(),
        };
        SignedTransaction::new(sign, transaction)
    }
}
