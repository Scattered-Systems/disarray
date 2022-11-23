/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{event::*, specs::*, variants::*};

pub(crate) mod event;

pub(crate) mod specs {
    use serde::Serialize;
    use std::fmt::Display;

    pub trait Eventful: Clone + Default + Display + Serialize {
        fn message(&self) -> String;
        fn timestamp(&self) -> i64;
    }
}

pub(crate) mod variants {
    use super::{Event, Eventful};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub enum Events<T: Eventful = Event> {
        GenericEvent(T),
        GenesisEvent(T),
        Idle,
    }

    impl<T: Eventful> Default for Events<T> {
        fn default() -> Self {
            Self::Idle
        }
    }

    impl<T: Eventful> std::fmt::Display for Events<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
        }
    }
}
