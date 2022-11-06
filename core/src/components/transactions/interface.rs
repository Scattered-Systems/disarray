/*
   Appellation: interface <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{BlockNc, BlockTs};
use scsys::prelude::H160;
use std::string::ToString;

pub trait Transactable<T: ToString> {
    fn message(&self) -> &T;
    fn nonce(&self) -> BlockNc;
    fn recv(&self) -> H160;
    fn timestamp(&self) -> BlockTs;
    fn value(&self) -> usize;
}

pub trait TransactionWrapper<T: ToString>: Transactable<T> {}

pub trait TransactionWrapperExt<T: ToString>: TransactionWrapper<T> {}
