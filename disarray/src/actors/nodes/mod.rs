/*
   Appellation: nodes <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use node::*;
use scsys::BoxError;

mod node;

pub trait NodeSpec<Act, Conf, Cont, Data> {
    fn actor(&self, address: libp2p::Multiaddr) -> Result<Act, BoxError>
        where
            Self: Sized;
    fn constructor(&self, config: Conf) -> Result<Self, BoxError>
        where
            Self: Sized;
    fn context(&self) -> Result<Cont, BoxError>
        where
            Self: Sized;
    fn data(&self) -> Result<Vec<Data>, BoxError>
        where
            Self: Sized;
}
