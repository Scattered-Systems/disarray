/*
   Appellation: core <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use behaviours::*;
pub use clients::*;
pub use configurations::*;
pub use consensus::*;
pub use primitives::*;
pub use utils::*;

mod behaviours;
mod clients;
mod configurations;
mod consensus;
mod primitives;

mod utils {
    pub fn timestamp_local() -> crate::Timestamps {
        crate::Timestamps::Standard(chrono::Local::now().timestamp())
    }

    pub fn timestamp_utc() -> crate::Timestamps {
        crate::Timestamps::Standard(chrono::Utc::now().timestamp())
    }
}
