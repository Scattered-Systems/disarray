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
}
