/*
   Appellation: actors <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::misc::*;

pub mod miners;
pub mod stakers;
pub mod validators;

pub(crate) mod misc {
    use scsys::{prelude::*, Hashable};
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum OperatingModes {
        Paused,
        Run(usize),
        Terminate,
    }

    impl std::fmt::Display for OperatingModes  {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
    }

    impl Default for OperatingModes {
        fn default() -> Self {
            Self::Paused
        }
    }
}
