/*
   Appellation: primitives <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;
pub use variants::*;

mod constants {
    ///
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {
    pub type BlockData = Vec<String>;
    pub type KademliaMS = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
}

mod variants {
    use super::*;

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Timestamps {
        Binary(bson::DateTime),
        Standard(scsys::BlockTs),
    }

    impl std::fmt::Display for Timestamps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
