/*
   Appellation: parts <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::nodes::Node;

use std::string::ToString;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Layer<T: ToString>(Vec<Node<T>>);
