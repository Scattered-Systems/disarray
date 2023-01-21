/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use acme::prelude::Eventful;
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::{fnl_remove, SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Event {
    pub event: Events,
    pub timestamp: i64
}

impl Event {
    pub fn new(event: Events) -> Self {
        let timestamp = Timestamp::default().into();
        Self { event, timestamp }
    }
}

impl From<Events> for Event {
    fn from(e: Events) -> Event {
        Event::new(e)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Events {
    #[default]
    Idle = 0,
    Genesis = 1,

    Update = 9,
}

impl From<Events> for i64 {
    fn from(e: Events) -> i64 {
        e as i64
    }
}

impl From<Event> for Events {
    fn from(e: Event) -> Events {
        e.event()
    }
}

impl Eventful for Event {
    type Event = Events;

    fn event(&self) -> Self::Event {
        self.event.clone()
    }
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}
