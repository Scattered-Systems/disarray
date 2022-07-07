/*
   Appellation: data
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::data::utils::*;

mod utils {
    pub fn timestamp_local() -> crate::Dates {
        crate::Dates::Standard(chrono::Local::now().timestamp())
    }

    pub fn timestamp_utc() -> crate::Dates {
        crate::Dates::Standard(chrono::Utc::now().timestamp())
    }
}
