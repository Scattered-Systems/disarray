/*
    Appellation: stateful <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::messages::Message;
use serde::Serialize;

pub trait Stateful: Clone + Default + Serialize + std::fmt::Display {
    type Data: std::fmt::Display;
    fn agency(&self) -> String;
    fn message(&self) -> &Message<Self::Data>;
    fn timestamp(&self) -> i64;
}

pub trait StatefulExt: Stateful {
    fn catalyst<S, T>(&mut self, f: &dyn Fn(S) -> T) -> Vec<T>;
    fn tags(&self) -> Vec<String>;
}
