/*
   Appellation: nodes
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use node::*;

mod node;

pub trait NodeSpec<Act, Conf, Cont, Data> {
    fn actor(&self, address: libp2p::Multiaddr) -> Result<Act, crate::CommonError>
        where
            Self: Sized;
    fn constructor(&self, config: Conf) -> Result<Self, crate::CommonError>
        where
            Self: Sized;
    fn context(&self) -> Result<Cont, crate::CommonError>
        where
            Self: Sized;
    fn datastore(&self, data: Vec<Data>) -> Result<Vec<Data>, crate::CommonError>
        where
            Self: Sized;
}
