/*
   Appellation: actors <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{locks::*, misc::*};

pub mod miners;
pub mod stakers;
pub mod validators;

pub type ControlChannel = crossbeam::channel::Receiver<ControlSignal>;

pub(crate) mod locks {
    use std::sync::{Arc, Mutex};

    /// Utility for creating uniform, thread-safe locks
    pub struct Lock<T>(pub Arc<Mutex<T>>);

    impl<T> Lock<T> {
        pub fn new(data: T) -> Self {
            Self(Arc::new(Mutex::from(data)))
        }
    }

    impl<T> std::convert::From<&T> for Lock<T>
    where
        T: Clone,
    {
        fn from(data: &T) -> Self {
            Self(Arc::new(Mutex::from(data.clone())))
        }
    }

    impl<T> Clone for Lock<T> {
        fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
    }
}

pub(crate) mod misc {
    use scsys::{prelude::*, Hashable};
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(
        Clone,
        Debug,
        Deserialize,
        EnumString,
        EnumVariantNames,
        Eq,
        Hash,
        Hashable,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
    )]
    pub enum ContextUpdateSignal {
        // it means external pow block comes
        NewBlock,
    }

    impl std::fmt::Display for ContextUpdateSignal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
        }
    }

    impl Default for ContextUpdateSignal {
        fn default() -> Self {
            Self::NewBlock
        }
    }

    #[derive(
        Clone,
        Debug,
        Deserialize,
        EnumString,
        EnumVariantNames,
        Eq,
        Hash,
        Hashable,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
    )]
    pub enum ControlSignal {
        Start(u64), // the number controls the lambda of interval between block generation
        Exit,
    }

    impl ControlSignal {
        pub fn start(data: u64) -> Self {
            Self::Start(data)
        }
        pub fn exit() -> Self {
            Self::Exit
        }
    }

    impl std::fmt::Display for ControlSignal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
        }
    }

    impl Default for ControlSignal {
        fn default() -> Self {
            Self::Exit
        }
    }

    #[derive(
        Clone,
        Debug,
        Deserialize,
        EnumString,
        EnumVariantNames,
        Eq,
        Hash,
        Hashable,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
    )]
    pub enum OperatingModes {
        Paused,
        Run(usize),
        Terminate,
    }

    impl std::fmt::Display for OperatingModes {
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
