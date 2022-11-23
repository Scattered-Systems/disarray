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
    pub enum OperatingModes<T> {
        Paused,
        Run(T),
        Terminate
    }

}


