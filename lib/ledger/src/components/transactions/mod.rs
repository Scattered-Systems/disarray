/*
   Appellation: transactions <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{misc::*, signed::*, specs::*, transaction::*, utils::*};

pub(crate) mod misc;
pub(crate) mod signed;
pub(crate) mod transaction;

pub(crate) mod utils {
    use super::{sign, Sign, SignedTransaction, Transaction};
    use rand::{self, Rng};
    use ring::signature::{
        self, Ed25519KeyPair, EdDSAParameters, KeyPair, Signature, UnparsedPublicKey,
        VerificationAlgorithm, ED25519,
    };
    use scsys::prelude::{generate_random_hash, random_keypair};

    /// Validate the authenticity of a transaction's signature
    pub fn validate_transaction_signature(
        trx: &Transaction,
        key: &Ed25519KeyPair,
        sig: &signature::Signature,
    ) -> bool {
        let ppk = UnparsedPublicKey::new(&ED25519, key.public_key().as_ref());
        ppk.verify(trx.to_string().as_bytes(), sig.as_ref()).is_ok()
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
        let transaction = t.trx.clone();
        let pubk = t.sig.pubk.clone();
        let sig = t.sig.sig.clone();
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
        let pubk = random_keypair();
        let sig = sign(&pubk, &transaction);
        let sign = Sign::new(pubk.public_key().as_ref().to_vec(), sig.as_ref().to_vec());
        SignedTransaction::new(sign, transaction)
    }
}
