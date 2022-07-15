/*
   Appellation: data
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use containers::*;
pub use models::*;
pub use schemas::*;
pub use utils::*;

mod containers;
mod models;
mod schemas;

mod utils {
    pub fn timestamp_local() -> crate::Timestamps {
        crate::Timestamps::Standard(chrono::Local::now().timestamp())
    }

    pub fn timestamp_utc() -> crate::Timestamps {
        crate::Timestamps::Standard(chrono::Utc::now().timestamp())
    }
}
