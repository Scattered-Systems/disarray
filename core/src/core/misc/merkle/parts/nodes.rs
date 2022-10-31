/*
   Appellation: nodes <parts> [merkle]
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use std::string::ToString;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node<T: ToString> {
    Leaf(T),
    Node(Box<Node<T>>, Box<Node<T>>),
}

impl<T: ToString> Node<T> {
    pub fn new_leaf(data: T) -> Self {
        Self::Leaf(data)
    }
}
