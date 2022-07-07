/*
   Appellation: mod
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use vaults::*;

mod vaults;

pub trait Schemable<Addr, Conf, Cont, Data> {
    fn constructor(&self, address: Addr, config: Conf, data: Data) -> Cont
        where
            Self: Sized;
}
