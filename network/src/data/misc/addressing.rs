/*
   Appellation: addressing <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use std::string::ToString;

pub trait Addressable: ToString {
    fn address(&self) -> &Self {
        self
    }
}
